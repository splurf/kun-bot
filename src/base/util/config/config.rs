use {
    super::{admins::Admins, title::Title},
    std::{
        collections::HashSet,
        fmt::Debug,
        fs::File,
        io::Error,
        path::{Path, PathBuf},
    },
    {serde::Deserialize, serde_json::from_reader},
};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    path: PathBuf,
    title: Title,
    admins: Admins,
}

impl<'de> Deserialize<'de> for Title {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

impl Config {
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(path: P, title: S) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            title: title.into(),
            admins: Admins::new(),
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn title(&self) -> String {
        self.title.inner()
    }

    pub fn admins(&self) -> &HashSet<u64> {
        self.admins.set()
    }
    pub fn load() -> Result<Self, Error> {
        from_reader(File::open("config.json")?).map_err(|e| e.into())
    }
}

impl<P: AsRef<Path>, S: AsRef<str>> From<(P, S)> for Config {
    fn from((path, title): (P, S)) -> Self {
        Self::new(path, title)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new("images", "")
    }
}
