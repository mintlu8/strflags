//! A string-enum and string-flags with fixed variants that can also accept arbitrary data.
//!
//! This is more extensible than traditional enums, bitflags or enumset,
//! while being more ergonomic and more typo-resiliant than a set of strings like `HashSet<String>`.
//!
//! # Example
//!
//! ```
//! # use ::strflags::*;
//! str_flags! {
//!     pub Color: [
//!         Red,
//!         Green,
//!         DarkBlue,
//!     ]
//! }
//! ```
//!
//! ### This Generates
//!
//! `pub struct Color(..)`
//!
//! And consts:
//!
//! * `pub const Red: Color = "red";`
//! * `pub const Green: Color = "green";`
//! * `pub const DarkBlue: Color = "darkblue";`
//!
//! **Note:** The implementation is not `&'static str` and subject to change.
//!
//! ### And implements
//!
//! * [`Debug`], [`Clone`], [`Eq`], [`Hash`]
//! * [`Display`](std::fmt::Display)
//! * [`FromStr`](std::str::FromStr)
//! * [`Borrow<str>`](std::borrow::Borrow)
//! * [`AsRef<str>`]
//! * [`PartialEq<impl AsRef<str>>`]
//! * [`Into<Flags<Self>>`]
//! * [`BitOr<Output = Flags<Self>>`](std::ops::BitOr)
//! * [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html), with the `serde` feature enabled
//! * [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html), with the `serde` feature enabled
//!
//! ### Flags
//!
//! You can create a `Flags<Color>` like this
//!```
//! # use ::strflags::*;
//! # str_flags! { Color: [ Red,  Green, DarkBlue ] };
//! let flags = Color::Red | Color::Green | Color::new("Yellow");
//! assert!(flags.contains(Color::Red));
//! assert!(flags.contains(Color::new("Green")));
//! assert!(flags.contains(Color::new("yELLOW")));
//! assert!(!flags.contains(Color::DarkBlue));
//! assert!(!flags.contains(Color::new("Black")));
//! ```
//!
//! # Format
//!
//! We stores all data in [`flatlowercase`](https://docs.rs/convert_case/latest/convert_case/enum.Case.html#variant.Flat)
//! to avoid case mismatches and some typos.
//!
//! # Serialization and Deserialization
//!
//! We use a string to serialize our "string enums".
//!
//! e.g. `"red"`
//!
//! We use a csv string to serialize our [`Flags`](crate::Flags).
//!
//! e.g. `"dog|cat|giraffe"`
//!
//! This ensures converting from enum to [`Flags`](crate::Flags) does not break serialization formats.
//!
//! # The `debug` feature
//!
//! Allowing any string to be an enum variant is obviously prone to typos.
//! When the debug feature is enabled, we performs a fuzzy string check
//! every time a comparison is made and emit a [`warn!`](https://docs.rs/log/latest/log/macro.warn.html)
//! though the [`log`](https://docs.rs/log/latest/log/) crate
//! if similar strings are found.
//!
//! This is obviously slow so be careful when using this feature.
//!
mod set;
mod operators;
mod query;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

pub use set::{Flags, FlagsMarker};
pub use query::Query;
#[doc(hidden)]
pub use ecow::EcoString;
#[doc(hidden)]
pub use identconv::lower_strify;
#[doc(hidden)]
pub use convert_case;


static MATCH_RATIO: AtomicUsize = AtomicUsize::new(8);

/// Sets the debug warning ratio, by default 8.
///
/// If the Levenshtein distance (number of edits) is less or equal to
/// `sum_of_len / ratio` during compare, emit a "maybe typo" warning
/// using the `log` crate.
///
/// Only has an effect if the `debug` feature is enabled.
pub fn set_debug_match_ratio(value: usize) {
    MATCH_RATIO.store(value, Relaxed)
}

#[cfg(not(feature = "debug"))]
#[doc(hidden)]
#[inline(always)]
pub fn str_eq(a: &str, b: &str) -> bool {
    a == b
}

#[cfg(feature = "debug")]
#[doc(hidden)]
#[inline(always)]
pub fn str_eq(a: &str, b: &str) -> bool {
    if a.len() > 3 && b.len() > 3 {
        if levenshtein::levenshtein(a, b) <= (a.len() + b.len()) / MATCH_RATIO.load(Relaxed) {
            log::warn!("{} and {} are similar, maybe a typo?", a, b)
        }
    }
    a == b
}

/// Construct a string enum.
/// 
/// To use [`Flags`], call [`str_flags`] instead.
///
/// This struct stores all its data in `flatlowercase` to avoid some typos.
///
/// Currently we use [`EcoString`](https://docs.rs/ecow/latest/ecow/string/struct.EcoString.html)
/// which can only inline 15 bytes. Having a larger compile time constant string
/// is a ***runtime*** error if the constant is **USED**.
///
/// However loading a larger string at runtime is fine.
///
/// This behavior will be changed in the future.
#[macro_export]
macro_rules! str_enum {
    ($(#[$main_attr:meta])* $vis:vis $name: ident: [$($(#[$attr: meta])* $fields: ident),* $(,)?]) => {
        
        #[derive(Debug, Clone, Eq, Hash)]
        $(#[$main_attr])*
        $vis struct $name(::strflags::EcoString);

        const _: () = {

            #[allow(non_upper_case_globals)]
            impl $name {
                $($(#[$attr])*
                pub const $fields: Self = Self(::strflags::EcoString::inline(
                    ::strflags::lower_strify!($fields)
                ));)*

                #[inline]
                pub fn new(s: &str) -> Self {
                    use ::strflags::convert_case::{Casing, Case::*};
                    Self(s.to_case(Flat).into())
                }
            }

            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.0.as_str())
                }
            }

            impl ::std::str::FromStr for $name {
                type Err=::std::convert::Infallible;

                fn from_str(s: &str) -> Result<Self, Self::Err>{
                    Ok(Self::new(s))
                }
            }

            impl Into<::strflags::Flags<Self>> for $name {
                fn into(self) -> ::strflags::Flags<Self> {
                    ::strflags::Flags::new(self)
                }
            }

            impl std::borrow::Borrow<str> for $name {
                fn borrow(&self) -> &str {
                    use std::borrow::Borrow;
                    self.0.borrow()
                }
            }

            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    self.0.as_ref()
                }
            }

            impl PartialEq<str> for $name {
                fn eq(&self, other: &str) -> bool {
                    self.0 == other
                }
            }

            impl<T: AsRef<str>> ::std::cmp::PartialEq<T> for $name {
                fn eq(&self, other: &T) -> bool {
                    ::strflags::str_eq(self.0.as_ref(), other.as_ref())
                }
            }

        };

        ::strflags::impl_serde!($name);
    };
}

/// Construct a string enum and enable [`Flags`] usage.
///
/// This provides all functionalites of [`str_enum`].
#[macro_export]
macro_rules! str_flags {
    ($(#[$main_attr: meta])* $vis:vis $name: ident: [$($(#[$attr: meta])* $fields: ident),* $(,)?]) => {
        ::strflags::str_enum!($(#[$main_attr])* $vis $name: [$($(#[$attr])* $fields),*]);

        const _: () = {
            impl ::strflags::FlagsMarker for $name {}

            impl ::std::ops::BitOr for $name {
                type Output = ::strflags::Flags<Self>;
                fn bitor(self, rhs: Self) -> Self::Output {
                    ::strflags::Flags::pair(self, rhs)
                }
            }

            impl ::std::ops::BitOr<::strflags::Flags<Self>> for $name {
                type Output = ::strflags::Flags<Self>;
                fn bitor(self, rhs: ::strflags::Flags<Self>) -> Self::Output {
                    rhs | self
                }
            }

            impl ::std::ops::BitAnd<::strflags::Flags<Self>> for $name {
                type Output = ::strflags::Flags<Self>;
                fn bitand(self, rhs: ::strflags::Flags<Self>) -> Self::Output {
                    rhs & self
                }
            }

            impl ::std::ops::BitXor<::strflags::Flags<Self>> for $name {
                type Output = ::strflags::Flags<Self>;
                fn bitxor(self, rhs: ::strflags::Flags<Self>) -> Self::Output {
                    rhs ^ self
                }
            }
        };
    }
}

#[cfg(not(feature="serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_serde {
    ($name: ident) => {}
}


#[cfg(feature="serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_serde {
    ($name: ident) => {
        const _: () = {
            use ::serde::{Serialize, Deserialize};
            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                    self.0.serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                    let s = <::std::borrow::Cow<str>>::deserialize(deserializer)?;
                    Ok(Self::new(s.as_ref()))
                }
            }
        };
    }
}
