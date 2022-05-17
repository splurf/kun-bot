use {
    super::{
        config::Config,
        id::Id,
        image::{Image, RawImage},
    },
    rand::{prelude::IteratorRandom, thread_rng},
    serenity::model::id::UserId,
    std::{
        collections::{HashMap, HashSet, VecDeque},
        io::Result,
        path::PathBuf,
    },
    tokio::fs::{read_dir, remove_file, rename},
};

#[derive(Debug)]
pub struct Kuns {
    images: HashMap<u16, RawImage>,
    vacant: VecDeque<Id>,
    config: Config,
    count: usize,
}

impl Kuns {
    /**
     * Create a new instance of `Kuns` with specified configurations
     */
    pub fn new(config: Config) -> Self {
        Self {
            images: HashMap::default(),
            vacant: VecDeque::default(),
            config,
            count: 0,
        }
    }

    /**
     * Check if the provided `user_id` is a valid admin within the `Kuns` system
     */
    pub fn is_authorized(&self, user_id: UserId) -> bool {
        self.config.admins().contains(&user_id.0)
    }

    /** Returns the next Id by incrementing the current `count` or popping any vacant Id's from the list of vacancies */
    fn next_id(&mut self) -> Id {
        if let Some(id) = self.vacant.pop_front() {
            id
        } else {
            let count = self.count;
            self.count += 1;
            count.into()
        }
    }

    /**
     * Return a randomly selected `RawImage` in the form of a fully embedded `Message`
     */
    pub fn random_kun(&mut self) -> Option<&mut RawImage> {
        self.images.values_mut().choose(&mut thread_rng())
    }

    /**
     * Remove the `RawImage` at the location of the key `k` if it exists then push the `Id` of that value to the list of vacancy Id's for later use
     */
    pub async fn remove(&mut self, k: &u16) -> Option<RawImage> {
        let image = self.images.remove(k)?;
        remove_file(image.path()).await.ok()?;
        self.vacant.push_back(image.id());
        Some(image)
    }

    async fn insert(&mut self, path: PathBuf) -> Option<RawImage> {
        let id = self.next_id();
        let file_name = id.file_name(path.extension()?);
        let new = path.with_file_name(file_name.clone());
        rename(path, new.clone()).await.ok()?;
        self.images.insert(
            id.clone().into(),
            RawImage::new(new, id, self.config.title())?,
        )
    }

    /**
     * Rescan over the images directory adding any newly found images and removing any existing images that weren't found
     */
    pub async fn load(&mut self) -> Result<()> {
        /* Any pre-existing images not marked should be removed as they no longer exist in the searched directory */
        let mut marks = HashSet::<u16>::default();

        /* Store each valid new image as well as marking any pre-existing images in the process */

        let mut rd = read_dir(self.config.path()).await?;
        let mut entries = Vec::new();

        while let Some(de) = rd.next_entry().await? {
            entries.push(de)
        }

        let new = entries
            .into_iter()
            .filter_map(|de| {
                let path = de.path();

                if RawImage::is_image(path.extension()?)? && {
                    if let Some(key) = || -> Option<u16> {
                        Some(Id::from_str(path.file_stem()?.to_str()?)?.into())
                    }() {
                        if self.images.contains_key(&key) {
                            marks.insert(key);
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                } {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>();

        /* Filter each already existing image then remove the leftovers */
        for k in self
            .images
            .iter()
            .filter_map(|(k, _)| {
                if !marks.contains(k) {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<u16>>()
        {
            //  Removing without context results in the dropping of any related messages instead of them being deleted
            self.remove(&k).await;
        }

        /* Insert the new images into the database */

        Ok(for path in new {
            self.insert(path).await;
        })
    }
}

impl Default for Kuns {
    fn default() -> Self {
        Self::new(Config::new("images", ""))
    }
}

impl From<Config> for Kuns {
    fn from(config: Config) -> Self {
        Self::new(config)
    }
}
