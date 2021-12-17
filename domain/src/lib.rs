//! # Domain
//!
//! The domain of this projects describes the shape of data
//! that helps self-employed people organize their lives.
//!
//! ## Entity IDs
//!
//! Most projects that follow [DDD](https://en.wikipedia.org/wiki/Domain-driven_design)
//! use IDs within the domain layer.\
//! Nevertheless, there is also the view that
//! [you should not use IDs in your domain entities](https://enterprisecraftsmanship.com/posts/dont-use-ids-domain-entities/),
//! but [references](https://enterprisecraftsmanship.com/posts/link-to-an-aggregate-reference-or-id/).
//! The problem with references is that you either have to fully load all the associated data of an entity or
//! rely on a lazy loading technique. However, the latter would create a dependency on a persistence layer,
//! which must not be the case.
//!
//! One can also see it in such a way that references are finally also only IDs,
//! which keep a unique memory address.
//!
//! In this project, therefore, all entites (or root aggregates) use an ID.

mod entity;

pub use self::entity::{area_of_life::AreaOfLife, thought::Thought, *};
