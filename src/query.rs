use std::borrow::Cow;
use std::marker::PhantomData;

use crate::EcoString;
use crate::Flags;
use crate::FlagsMarker;

/// Items that can be used in [`Flags::contains()`](crate::Flags::contains)
/// 
/// # Warning
/// 
/// Although a few string types are supported,
/// their usage is not encouraged, since
/// we do not perform case conversion in `contains`.
/// 
/// The user should take care to proivde only `flatlowercase` strings.
pub trait Query<T: FlagsMarker> {
    type Iter<'t>: Iterator<Item = &'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t>;
}

impl<T: FlagsMarker> Query<T> for str {
    type Iter<'t> = std::iter::Once<&'t str>;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self)
    }
}

impl<T: FlagsMarker> Query<T> for &str {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self)
    }
}

impl<T: FlagsMarker> Query<T> for &&str {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self)
    }
}

impl<T: FlagsMarker> Query<T> for String {
    type Iter<'t> = std::iter::Once<&'t str>;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_str())
    }
}

impl<T: FlagsMarker> Query<T> for &String {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_str())
    }
}

impl<'a, T: FlagsMarker> Query<T> for Cow<'a, str> {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_ref())
    }
}

impl<'a, 'b, T: FlagsMarker> Query<T> for &'b Cow<'a, str> {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_ref())
    }
}


impl<T: FlagsMarker> Query<T> for EcoString {
    type Iter<'t> = std::iter::Once<&'t str>;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_str())
    }
}

impl<T: FlagsMarker> Query<T> for &EcoString {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_str())
    }
}

impl<T: FlagsMarker> Query<T> for T {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_ref())
    }
}

impl<T: FlagsMarker> Query<T> for &T {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_ref())
    }
}

impl<T: FlagsMarker> Query<T> for &&T {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_ref())
    }
}

impl<T: FlagsMarker> Query<T> for &&&T {
    type Iter<'t> = std::iter::Once<&'t str> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        std::iter::once(self.as_ref())
    }
}


pub struct AsRefStrIter<'t, T>(pub(crate) T, pub(crate) PhantomData<&'t T>);

impl<'t, T, K> Iterator for AsRefStrIter<'t, T> 
        where T: Iterator<Item = &'t K>, K: AsRef<str> + 't {
    type Item = &'t str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|s| (*s).as_ref())
    }
}

impl<T: FlagsMarker + PartialEq> Query<T> for &[&str] {
    type Iter<'t> = AsRefStrIter<'t, std::slice::Iter<'t, &'t str>> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        AsRefStrIter(self.iter(), PhantomData)
    }
}


impl<'a, T: FlagsMarker + PartialEq> Query<T> for &'a [T] {
    type Iter<'t> = AsRefStrIter<'t, std::slice::Iter<'t, T>> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        AsRefStrIter(self.iter(), PhantomData)
    }
}

impl<T: FlagsMarker + PartialEq> Query<T> for Vec<T> {
    type Iter<'t> = AsRefStrIter<'t, std::slice::Iter<'t, T>> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        AsRefStrIter(self.iter(), PhantomData)
    }
}

impl<'a, T: FlagsMarker + PartialEq> Query<T> for &'a Vec<T> {
    type Iter<'t> = AsRefStrIter<'t, std::slice::Iter<'t, T>> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        AsRefStrIter(self.iter(), PhantomData)
    }
}

impl<T: FlagsMarker + PartialEq> Query<T> for Flags<T> {
    type Iter<'t> = AsRefStrIter<'t, std::slice::Iter<'t, T>> where T: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        AsRefStrIter(self.0.iter(), PhantomData)
    }
}

impl<T: FlagsMarker + PartialEq> Query<T> for &Flags<T> {
    type Iter<'t> = AsRefStrIter<'t, std::slice::Iter<'t, T>> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        AsRefStrIter(self.0.iter(), PhantomData)
    }
}

impl<T: FlagsMarker + PartialEq> Query<T> for &&Flags<T> {
    type Iter<'t> = AsRefStrIter<'t, std::slice::Iter<'t, T>> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        AsRefStrIter(self.0.iter(), PhantomData)
    }
}

impl<T: FlagsMarker + PartialEq> Query<T> for &&&Flags<T> {
    type Iter<'t> = AsRefStrIter<'t, std::slice::Iter<'t, T>> where Self: 't;
    fn items<'t>(&'t self) -> Self::Iter<'t> {
        AsRefStrIter(self.0.iter(), PhantomData)
    }
}