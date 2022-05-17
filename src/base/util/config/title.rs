use {serde::Deserialize, std::fmt::Debug};

const DEFAULT_HONORIFIC: &str = "Kun";

#[derive(Clone, Debug)]
pub struct Title(String);

impl Title {
    pub fn inner(&self) -> String {
        self.0.clone()
    }
}

impl<S: AsRef<str>> From<S> for Title {
    fn from(s: S) -> Self {
        let mut title = s.as_ref().to_string();
        let mut honorific = String::from(DEFAULT_HONORIFIC);

        if !title.is_empty() {
            title.push('-');
            honorific = honorific.to_lowercase()
        }
        title.push_str(&honorific);
        Self(title)
    }
}

impl<'de> Deserialize<'de> for Title {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}
