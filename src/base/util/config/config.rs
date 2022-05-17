use {
    super::{admins::Admins, title::Title},
    std::{
        collections::HashSet,
        fmt::Debug,
        fs::File,
        io::{Error, ErrorKind},
        path::{Path, PathBuf},
    },
    {serde::Deserialize, serde_json::from_reader},
};

const CONFIG_PATH: &str = "config.json";

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    path: PathBuf,
    title: Title,
    admins: Admins,
}

impl Config {
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(path: P, title: S) -> Result<Self, Error> {
        Self {
            path: path.as_ref().into(),
            title: title.into(),
            admins: Admins::new(),
        }
        .check()
    }

    fn check(self) -> Result<Self, Error> {
        let exists = self.path.exists();
        let is_dir = self.path.is_dir();

        if exists && is_dir {
            Ok(self)
        } else if exists {
            Err(ErrorKind::InvalidData.into())
        } else {
            Err(ErrorKind::NotFound.into())
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
        let result: Result<Config, Error> =
            from_reader(File::open(CONFIG_PATH)?).map_err(Into::into);
        result?.check()
    }
}
