# Clean Architecture with Rust

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

The main purpose of this example is to discuss the architecture
not the application szenario itself.
So let's implement a simple application that manages some `Item`s
which hold some data. For simplicity the data itself is just
a `title` and nothing else.