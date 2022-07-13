use std::{
    ffi::{OsStr, OsString},
    ops::Deref,
    path::Path,
};

const LIMIT: usize = 4;
const OVERFLOW: usize = 10_000;

#[derive(Clone, Debug)]
pub struct Id(u16);

impl Id {
    pub fn new(i: usize) -> Self {
        if i < OVERFLOW {
            Self(i as u16)
        } else {
            panic!("Id overflow (too many kuns need to expand!!!)")
        }
    }

    pub fn from_path<T: AsRef<Path>>(s: T) -> Option<Id> {
        let s = s.as_ref().file_stem()?.to_str()?.trim();

        if s.len() <= LIMIT {
            Some(Self(s.parse().ok()?))
        } else {
            None
        }
    }

    pub fn file_name(&self, extension: &OsStr) -> OsString {
        let mut s: OsString = format!(
            "{}{}.",
            "0".repeat(LIMIT - self.0.to_string().len()),
            self.0
        )
        .into();
        s.push(&extension);
        s
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Deref for Id {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
