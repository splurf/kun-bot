use {
    rand::{seq::SliceRandom, thread_rng},
    serenity::{
        async_trait,
        builder::CreateMessage,
        client::{Context, EventHandler},
        model::{channel::Message, prelude::Ready},
        utils::Color,
        Result,
        {prelude::GatewayIntents, Client},
    },
    std::{
        env::{args, var},
        fs::read_dir,
        path::{Path, PathBuf},
    },
};

const COMMAND: &str = "s.w";
const COLOR: Color = Color::from_rgb(0, 0, 0);

struct Image {
    file_name: String,
    title: String,
    path: PathBuf,
    id: u16,
}

impl Image {
    fn file_name(&self) -> &str {
        &self.file_name
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn id(&self) -> u16 {
        self.id
    }

    fn as_embed<'a, 'b>(&'b self, m: &'a mut CreateMessage<'b>) -> &'a mut CreateMessage<'b> {
        m.embed(|e| {
            e.title(self.title())
                .attachment(self.file_name())
                .color(COLOR)
                .footer(|f| f.text(self.id()))
        })
        .add_file(self.path())
    }
}

pub struct Bot(Vec<Image>);

impl Bot {
    pub fn new() -> Option<Self> {
        let mut id = 0;
        let mut args = args();

        let dir = args.nth(1)?;
        let title = args.next().get_or_insert("Kun".to_string()).clone();

        Some(Self(
            read_dir(dir)
                .ok()?
                .filter_map(|de| {
                    id += 1;
                    let path = de.ok()?.path();
                    let file_name = path.file_name()?.to_str()?.to_string();
                    let title = title.clone();

                    ["jpg", "   jpeg", "png"]
                        .contains(
                            &path
                                .extension()
                                .expect("Image without extension")
                                .to_str()?,
                        )
                        .then_some(Image {
                            file_name,
                            title,
                            path,
                            id,
                        })
                })
                .collect(),
        ))
    }

    pub async fn run(self) -> Result<()> {
        let token = var("KUN_BOT_TOKEN").expect("`KUN_BOT_TOKEN` environmental variable not found");

        Client::builder(
            token,
            GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES,
        )
        .event_handler(self)
        .await?
        .start_autosharded()
        .await
    }

    fn choose(&self) -> Option<&Image> {
        self.0.choose(&mut thread_rng())
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.as_str() == COMMAND {
            if let Some(image) = self.choose() {
                if let Err(e) = msg
                    .channel_id
                    .send_message(ctx.http, |m| image.as_embed(m))
                    .await
                {
                    println!("{:?}", e)
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.online().await;
    }
}
