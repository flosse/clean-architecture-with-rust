// The ID of this entity is not defined here
// because an artificial ID would leaky abstraction.
// Further readings:
// - https://enterprisecraftsmanship.com/posts/dont-use-ids-domain-entities/
// - https://enterprisecraftsmanship.com/posts/link-to-an-aggregate-reference-or-id/
#[derive(Debug)]
pub struct Item {
    pub title: String,
}
