use std::fmt::Debug;

#[derive(Clone)]
pub struct Title(String);

impl Title {
    pub fn inner(&self) -> String {
        self.0.clone()
    }
}

impl<S: AsRef<str>> From<S> for Title {
    fn from(s: S) -> Self {
        let mut title = s.as_ref().to_string();

        if !title.is_empty() {
            title.push('-')
        }
        title.push_str("Kun");
        Self(title)
    }
}

impl Debug for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.0).as_str())
    }
}
