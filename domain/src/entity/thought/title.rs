#[derive(Debug, Clone, PartialEq)]
pub struct Title(String);

impl Title {
    pub const fn new(title: String) -> Self {
        Self(title)
    }
}

impl From<Title> for String {
    fn from(from: Title) -> Self {
        from.0
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Within the domain, we define any domain-wide facts,
// but the actual verification is done in outer layers.

const MAX_TITLE_LEN: usize = 80;
const MIN_TITLE_LEN: usize = 3;

#[derive(Debug)]
pub struct TitleConstraints;

impl TitleConstraints {
    pub const fn min_len() -> usize {
        MIN_TITLE_LEN
    }
    pub const fn max_len() -> usize {
        MAX_TITLE_LEN
    }
}
