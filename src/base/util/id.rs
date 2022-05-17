use std::ffi::{OsStr, OsString};

const LIMIT: usize = 4;

#[derive(Clone, Debug)]
pub struct Id(u16);

impl Id {
    pub fn from_str<T: AsRef<str>>(s: T) -> Option<Id> {
        let s = s.as_ref().trim();

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

impl From<usize> for Id {
    fn from(i: usize) -> Self {
        if i < 10000 {
            Self(i as u16)
        } else {
            panic!("Value too large")
        }
    }
}

impl From<Id> for u16 {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
