# strflags

A string-enum and string-flags with fixed variants that can also accept arbitrary data.

This is more extensible than traditional enums, bitflags or enumset,
while being more ergonomic and more typo-resiliant than a set of strings like `HashSet<String>`.

## Example

```rust
str_flags! {
    #[derive(PartialOrd)]
    pub Color: [
        Red,
        Green,
        DarkBlue,
    ]
}
```

### This Generates

`pub struct Color(..)`

And consts:

* `pub const Red: Color = "red";`
* `pub const Green: Color = "green";`
* `pub const DarkBlue: Color = "darkblue";`

### And auto implements

* `Debug`, `Clone`, `Eq`, `Hash`
* `Display`
* `FromStr`
* `Borrow<str>`
* `AsRef<str>`
* `PartialEq<impl AsRef<str>>`
* `Into<Flags<Self>>`
* `BitOr<Output = Flags<Self>>`
* `Serialize`, with the `serde` feature enabled
* `Deserialize`, with the `serde` feature enabled

### Flags

You can create a `Flags<Color>` like this.

### `str_enum!()`

use the `str_enum!()` macro to opt out of flags related features.

```rust
let flags = Color::Red | Color::Green | Color::new("Yellow");
```

## Format

We stores all data in `flatlowercase`
to avoid case mismatches and some typos.

## Serialization and Deserialization

We use a string to serialize our "string enums".

e.g. `"red"`

We use a csv like string to serialize our `Flags`

e.g. `"dog|cat|giraffe"`

This ensures converting from enum to `Flags` does not break serialization formats.

## The `debug` feature

Allowing any string to be an enum variant is obviously prone to typos.
When the debug feature is enabled, we performs a fuzzy string check
every time a comparison is made and emit a `warn!`
though the `log` crate
if similar strings are found.
This is obviously slow so be careful when using this feature.

## Warning

Currently we use [`EcoString`](https://docs.rs/ecow/latest/ecow/string/struct.EcoString.html)
which can only inline 15 bytes. Having a larger compile time constant string
is a ***runtime*** error if the constant is **USED**.

A proc macro implementation in the future will address this issue.
