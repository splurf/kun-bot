use {
    crate::{Inner::InvalidPath, Result},
    rand::{seq::SliceRandom, thread_rng},
    serenity::{builder::CreateMessage, prelude::TypeMapKey, utils::Color},
    std::{
        fs::read_dir,
        path::{Path, PathBuf},
    },
};

pub struct Images;

impl Images {
    const EXTENSIONS: [&'static str; 3] = ["jpg", "jpeg", "png"];

    pub fn get_images(title: String, paths: Vec<PathBuf>) -> Result<<Self as TypeMapKey>::Value> {
        let mut id = 0;

        let images = paths
            .into_iter()
            .map(|p| -> Result<_> {
                let read_dir = read_dir(p)?;
                let filtered = read_dir.collect::<Result<Vec<_>, _>>()?;

                Ok(filtered
                    .into_iter()
                    .filter_map(|de| {
                        let path = de.path();
                        let file_name = path.file_name()?.to_str()?.to_string();
                        let title = title.clone();

                        let image = Self::EXTENSIONS
                            .contains(&path.extension()?.to_str()?)
                            .then_some(Image {
                                file_name,
                                title,
                                path,
                                id,
                            });
                        id += 1;
                        image
                    })
                    .collect::<Vec<Image>>())
            })
            .collect::<Result<Vec<Vec<Image>>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<Image>>();

        (images.len() > 0)
            .then_some(images)
            .ok_or(InvalidPath.into())
    }

    pub fn choose(images: &<Self as TypeMapKey>::Value) -> Option<&Image> {
        images.choose(&mut thread_rng())
    }
}

impl TypeMapKey for Images {
    type Value = Vec<Image>;
}

#[derive(Clone, Debug)]
pub struct Image {
    file_name: String,
    title: String,
    path: PathBuf,
    id: usize,
}

impl Image {
    const COLOR: Color = Color::from_rgb(0, 0, 0);

    fn file_name(&self) -> &str {
        &self.file_name
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn id(&self) -> usize {
        self.id
    }

    pub fn as_embed<'a, 'b>(&'b self, m: &'a mut CreateMessage<'b>) -> &'a mut CreateMessage<'b> {
        m.embed(|e| {
            e.title(self.title())
                .attachment(self.file_name())
                .color(Self::COLOR)
                .footer(|f| f.text(self.id()))
        })
        .add_file(self.path())
    }
}
