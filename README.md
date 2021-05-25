# Full-Stack Clean Architecture with Rust

This repository contains an example implementation of a
[Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
written in [Rust](https://rust-lang.org).

## Circles

Each circle (a.k.a layer) is organized in a separate create.
Currently there are these circles:

- `entity`
- `domain`
- `application`
- `adapter`
- `infrastructure`

Depending on your system the amount and the name of circles could
be different but the main **dependency rule** must be ensured:

> Source code dependencies can only point inwards.

that means

> Nothing in an inner circle can know anything at all about
> something in an outer circle

## Example Szenario

The main purpose of this example is to discuss the architecture,
not the application szenario itself.
Nevertheless, the code represents a real-world application
that helps self-employed people organize their lives.

### User Stories

> *As a* self-employed person,
> *I want to* be able to write down spontaneous thoughts,
> *so that* I can find them again at a central point
