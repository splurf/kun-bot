use {
    super::{cmd::Cmd, util::ImageMut, util::Kuns as RawKuns},
    crate::Config,
    serenity::{
        async_trait,
        client::{Context, EventHandler},
        model::{channel::Message, prelude::Ready},
        Result,
        {
            prelude::{GatewayIntents, RwLock},
            Client,
        },
    },
    std::{
        env::var,
        io::{Error as IoError, ErrorKind},
        sync::Arc,
    },
};

pub type Kuns = Arc<RwLock<RawKuns>>;

pub struct Bot(Kuns);

impl Bot {
    /**
     * Begin the event handler of the newly created `Client` with specified discord bot token
     */
    pub async fn run(self) -> Result<()> {
        let token =
            var("DISCORD_TOKEN").map_err(|e| IoError::new(ErrorKind::NotFound, e.to_string()))?;

        Client::builder(
            token,
            GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES,
        )
        .event_handler(self)
        .await?
        .start_autosharded()
        .await
    }

    async fn handle_command(&self, cmds: Vec<Cmd>, ctx: Context, msg: Message) -> Result<()> {
        let mut kuns = self.0.write().await;
        kuns.load().await?;

        if cmds.is_empty() {
            if let Some(kun) = kuns.random_kun() {
                let msg = msg
                    .channel_id
                    .send_message(&ctx.http, |m| kun.as_message(m))
                    .await?;
                kun.add_message(msg)
            } else {
                //  hleh
            }
        } else {
            for cmd in cmds {
                match cmd {
                    Cmd::Remove(v) => {
                        if kuns.is_authorized(msg.author.id) {
                            if let Some(mut image) = kuns.remove(&v).await {
                                while let Some(msg) = image.remove_message() {
                                    msg.delete(&ctx.http).await?
                                }
                            }
                        }
                        msg.delete(&ctx.http).await?
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for Bot {
    fn default() -> Self {
        Config::default().into()
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(cmds) = Cmd::from_content(msg.clone().content) {
            if let Err(e) = self.handle_command(cmds, ctx, msg).await {
                println!("{}", e)
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.online().await;
    }
}

impl From<Config> for Bot {
    fn from(config: Config) -> Self {
        Self(Arc::new(RwLock::new(config.into())))
    }
}
