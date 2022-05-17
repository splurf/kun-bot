use {serde::Deserialize, std::collections::HashSet};

#[derive(Clone, Debug)]
pub struct Admins(HashSet<u64>);

impl Admins {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn set(&self) -> &HashSet<u64> {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Admins {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Vec::deserialize(deserializer)?.into())
    }
}

impl From<Vec<u64>> for Admins {
    fn from(v: Vec<u64>) -> Self {
        let mut set = HashSet::new();
        v.into_iter().for_each(|n| {
            set.insert(n);
        });
        Self(set)
    }
}
