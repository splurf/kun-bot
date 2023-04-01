use {
    crate::res::{Image, Images},
    serde::Deserialize,
    serenity::{json::prelude::from_reader, prelude::TypeMapKey},
    simple_home_dir::expand_tilde,
    std::{fs::File, io::BufReader, path::PathBuf},
};

#[derive(Deserialize)]
struct Config {
    prefix: String,
    title: String,
    paths: Vec<PathBuf>,
}

pub fn get_config() -> Result<(Vec<Image>, RawConfigCache), String> {
    let file = File::open("config.json").map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let Config {
        mut prefix,
        mut title,
        paths,
    } = from_reader(reader).map_err(|e| e.to_string())?;

    if prefix.is_empty() {
        prefix = "s.".to_string()
    }
    if title.is_empty() {
        title = "kun-bot".to_string()
    }
    if paths.is_empty() {
        return Err("Missing image directory(s)".to_string());
    };
    let images = Images::get_images(
        title,
        paths
            .into_iter()
            .map(|p| expand_tilde(p))
            .collect::<Option<Vec<PathBuf>>>()
            .ok_or("Failed to expand image directory(s)")?,
    )?;
    Ok((images, RawConfigCache { prefix }))
}

pub struct ConfigCache;

impl TypeMapKey for ConfigCache {
    type Value = RawConfigCache;
}

pub struct RawConfigCache {
    prefix: String,
}

impl RawConfigCache {
    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }
}
