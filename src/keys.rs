use {
    crate::{ErrorKind::InvalidPath, Result},
    image::ImageFormat,
    rand::{seq::SliceRandom, thread_rng},
    serenity::{
        all::{GuildId, MessageId, UserId},
        builder::{CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage},
        model::Color,
        prelude::TypeMapKey,
    },
    std::{
        collections::HashMap,
        fs::{read_dir, DirEntry},
        path::{Path, PathBuf},
    },
};

pub struct Whitelist {
    data: Vec<GuildId>,
    path: PathBuf,
}

impl Whitelist {
    pub const fn new(data: Vec<GuildId>, path: PathBuf) -> Self {
        Self { data, path }
    }

    pub fn data(&self) -> &[GuildId] {
        self.data.as_slice()
    }

    pub fn data_mut(&mut self) -> &mut Vec<GuildId> {
        self.data.as_mut()
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl TypeMapKey for Whitelist {
    type Value = Self;
}

pub struct Admins;

impl TypeMapKey for Admins {
    type Value = Vec<UserId>;
}

pub struct MessageLink;

impl TypeMapKey for MessageLink {
    type Value = HashMap<MessageId, MessageId>;
}

pub struct Prefix(String);

impl Prefix {
    pub const fn new(prefix: String) -> Self {
        Self(prefix)
    }
}

impl AsRef<str> for Prefix {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl TypeMapKey for Prefix {
    type Value = Self;
}

pub struct Images;

impl Images {
    pub async fn get_images(
        title: &str,
        paths: Vec<PathBuf>,
    ) -> Result<<Self as TypeMapKey>::Value> {
        let mut id = 0;

        let check_de = |de: DirEntry| -> Option<(PathBuf, String)> {
            let path = de.path();
            // check if it's an image
            ImageFormat::from_extension(path.extension()?)?;
            Some((path.clone(), path.file_name()?.to_str()?.to_string()))
        };

        let mut images = Vec::new();

        for (p, file_name) in paths
            .clone()
            .into_iter()
            .filter_map(|p| Some(read_dir(p).ok()?.filter_map(Result::ok)))
            .flatten()
            .filter_map(check_de)
        {
            id += 1; // increment

            let cm = CreateMessage::default()
                .embed(
                    CreateEmbed::default()
                        .title(title)
                        .attachment(file_name)
                        .color(Color::from_rgb(0, 0, 0))
                        .footer(CreateEmbedFooter::new(id.to_string())),
                )
                .add_file(CreateAttachment::path(p).await?);
            images.push(cm)
        }

        (images.len() > 0)
            .then_some(images)
            .ok_or(InvalidPath.into())
    }

    pub fn choose(images: &<Self as TypeMapKey>::Value) -> Option<&CreateMessage> {
        images.choose(&mut thread_rng())
    }
}

impl TypeMapKey for Images {
    type Value = Vec<CreateMessage>;
}
