// NOTE:
// The ID of this entity is not defined here.
//
// TODO:
// How to express relations between entities?
// Should we add a generic ID?
//
//     pub struct Item<I> {
//         pub id: I,
//         pub title: String
//     }
//
// What would be the disadvantage of defining a concrete
// type within the entity circle?
//
//     pub struct ItemId(u64);
//
// Or should the relation expressed in the domain,
// with a separate type e.g. like `ItemFooRelation`?
#[derive(Debug)]
pub struct Item {
    pub title: String,
}
