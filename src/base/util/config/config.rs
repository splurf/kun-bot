use {
    super::{admins::Admins, title::Title},
    std::{
        collections::HashSet,
        env::{current_exe, set_current_dir, var, VarError},
        fmt::Debug,
        fs::File,
        io::{
            Error,
            ErrorKind::{InvalidData, NotFound},
        },
        path::{Path, PathBuf},
    },
    {serde::Deserialize, serde_json::from_reader},
};

const CONFIG_PATH: &str = "config.json";
const TARGET: &str = "target";
const HOME: &str = "HOME";

const CHECK_HOME: fn() -> Result<PathBuf, VarError> =
    || -> Result<PathBuf, VarError> { Ok(PathBuf::from(var(HOME)?)) };

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

    pub fn load() -> Result<Self, Error> {
        let ce = current_exe()?;
        let mut iter = ce.into_iter();

        while let Some(c) = iter.next_back() {
            if c == TARGET {
                break;
            }
        }
        set_current_dir(iter.collect::<PathBuf>())?;

        from_reader::<_, Config>(File::open(CONFIG_PATH)?)
            .map_err(Into::<Error>::into)?
            .check()
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

    fn check(mut self) -> Result<Self, Error> {
        if let Ok(mut home) = CHECK_HOME() {
            if !self.path.starts_with(&home) {
                home.push(&self.path);
                self.path = home;
            }
        }

        let exists = self.path.exists();
        let is_dir = self.path.is_dir();

        if exists && is_dir {
            Ok(self)
        } else {
            Err(if exists { InvalidData } else { NotFound }.into())
        }
    }
}
