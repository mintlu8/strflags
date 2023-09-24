use std::{fmt::Display, str::FromStr, marker::PhantomData};
use crate::{query::AsRefStrIter, Query};
use smallvec::SmallVec;

/// Marker for implementors of [`str_flags`](crate::str_flags).
pub trait FlagsMarker: AsRef<str> + PartialEq<str> {}

/// A set of string-enums
#[derive(Clone)]
pub struct Flags<T: PartialEq, const SEP: char='|'>(pub(crate) SmallVec<[T; 2]>);

impl<T: PartialEq, const S: char> Flags<T, S> {
    pub const EMPTY: Self = Self(SmallVec::new_const());

    #[inline(always)]
    pub fn new(value: T) -> Self {
        let mut vec = SmallVec::new_const();
        vec.push(value);
        Self(vec)
    }

    #[inline(always)]
    pub fn pair(value1: T, value2: T) -> Self {
        if value1 == value2{
            Self::new(value1)
        } else {
            Self(SmallVec::from_const([value1, value2]))
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }


    #[inline(always)]
    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn contains(&self, t: impl Query<T>) -> bool where T: FlagsMarker{
        for i in t.items() {
            if self.0.iter().any(|x| x == i) {
                return true;
            }
        }
        false
    }

    #[inline(always)]
    pub fn iter<'t>(&'t self) -> impl Iterator<Item = &'t T>{
        self.0.iter()
    }
}

impl<T: PartialEq, const S: char> Default for Flags<T, S> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<T: PartialEq, const S: char> PartialEq for Flags<T, S> where T: PartialEq{
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() ||
            self.iter().all(|x| other.iter().any(|y| x == y))
    }
}

impl<T: PartialEq, const S: char> IntoIterator for Flags<T, S> {
    type Item = T;
    type IntoIter = smallvec::IntoIter<[T; 2]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'t, T: PartialEq, const S: char> IntoIterator for &'t Flags<T, S> {
    type Item = &'t T;
    type IntoIter = std::slice::Iter<'t, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}


impl<T: PartialEq, const S: char> std::fmt::Debug for Flags<T, S> where T: AsRef<str>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        set.entries(AsRefStrIter(self.iter(), PhantomData));
        set.finish()?;
        Ok(())
    }
}

impl<T: PartialEq, const S: char> Display for Flags<T, S> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(item) = iter.next() {
            item.fmt(f)?;
        }
        for item in iter {
            S.fmt(f)?;
            item.fmt(f)?;
        }
        Ok(())
    }
}


impl<T: PartialEq, const S: char> FromStr for Flags<T, S> where T: FromStr {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr: Result<_, _> = s.split(S).map(T::from_str).collect();
        Ok(Self(arr?))
    }
}

#[cfg(feature="serde")]
const _: () = {
    use ::serde::{Serialize, Deserialize};

    impl<T: PartialEq, const SEP: char> Serialize for Flags<T, SEP> where T: Display {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            self.to_string().serialize(serializer)
        }
    }

    impl<'de, T: PartialEq, const SEP: char> Deserialize<'de> for Flags<T, SEP> where T: FromStr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
            let s = <std::borrow::Cow<str>>::deserialize(deserializer)?;
            let list: Result<_, _> = s.as_ref().split(SEP).map(|x| T::from_str(x)).collect();
            match list{
                Ok(list) => Ok(Self(list)),
                Err(_) => Err(serde::de::Error::custom(
                    format!("Invalid {}: \"{}\".", ::std::any::type_name::<Self>(), s))
                )
            }
        }
    }
};
