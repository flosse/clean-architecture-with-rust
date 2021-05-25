// The ID of this entity is not defined here
// because an artificial ID would leaky abstraction.
// Further readings:
// - https://enterprisecraftsmanship.com/posts/dont-use-ids-domain-entities/
// - https://enterprisecraftsmanship.com/posts/link-to-an-aggregate-reference-or-id/
#[derive(Debug, Clone)]
pub struct Thought {
    pub title: Title,
}

impl Thought {
    pub const fn new(title: String) -> Self {
        let title = Title::new(title);
        Self { title }
    }
}

#[derive(Debug, Clone)]
pub struct Title(String);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_new_thought() {
        let thought = Thought::new("foo".to_string());
        assert_eq!(thought.title.as_ref(), "foo");
    }
}
