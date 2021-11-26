# Nicht

A more natural way to use `not`.

> Might be easier to read in a voice that doesn't sound like Borat

## Motivation

Boolean conditions sometimes require mental gymnastics from the reader especially on more complicated expressions which may lead to [subtle bugs](https://arstechnica.com/gadgets/2021/07/google-pushed-a-one-character-typo-to-production-bricking-chrome-os-devices/).

Some languages have a `not` keyword available to reduce this kind of bugs especially with the sometimes hard to see single `!` negatiting whole expressions.

Unfortunately this is not the case in rust.

Instead the `std::ops::Not` can only be applied to the end of an expression making it hard to read again.

## Usage

This is the `std` way of using `not`:

```rust
use std::ops::Not;

if foo.some_condition().not() {
    // etc.
}
```

With this crate you can achieve the more natural way of reading code like `if not some condition` is true.

```rust
use nicht::not;

if not(foo.some_condition()) {
    // etc.
}
```

## Credit

[By u/killercup](https://www.reddit.com/r/rust/comments/8u8o5o/comment/e1dtvu0/)

