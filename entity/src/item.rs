// The ID of this entity is not defined here
// because an artificial ID would leaky abstraction.
// Further readings:
// - https://enterprisecraftsmanship.com/posts/dont-use-ids-domain-entities/
// - https://enterprisecraftsmanship.com/posts/link-to-an-aggregate-reference-or-id/
#[derive(Debug, Clone)]
pub struct Item {
    pub title: Title,
}

#[derive(Debug, Clone)]
pub struct Title(pub String);

impl Title {
    pub const fn new(title: String) -> Self {
        Self(title)
    }
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
