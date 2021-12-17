#[derive(Debug, Clone)]
pub struct Name(String);

impl Name {
    pub const fn new(name: String) -> Self {
        Self(name)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Name> for String {
    fn from(from: Name) -> Self {
        from.0
    }
}

const MAX_NAME_LEN: usize = 30;
const MIN_NAME_LEN: usize = 5;

#[derive(Debug)]
pub struct NameConstraints;

impl NameConstraints {
    pub const fn min_len() -> usize {
        MIN_NAME_LEN
    }
    pub const fn max_len() -> usize {
        MAX_NAME_LEN
    }
}
