use crate::Flags;
use smallvec::SmallVec;
use std::ops::*;

impl<T: PartialEq, const S: char> Add<T> for Flags<T, S> {
    type Output = Self;
    fn add(mut self, rhs: T) -> Self{
        if !self.0.iter().any(|x| x == &rhs){
            self.0.push(rhs)
        }
        self
    }
}

impl<T: PartialEq, const S: char> Sub<T> for Flags<T, S> {
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self{
        if let Some(index) = self.0.iter().position(|x| x == &rhs){
            self.0.remove(index);
        }
        self
    }
}

impl<T: PartialEq, const S: char> BitOr<T> for Flags<T, S> {
    type Output = Self;
    fn bitor(mut self, rhs: T) -> Self{
        if !self.0.iter().any(|x| x == &rhs){
            self.0.push(rhs)
        }
        self
    }
}

impl<T: PartialEq, const S: char> BitAnd<T> for Flags<T, S> {
    type Output = Self;
    fn bitand(self, rhs: T) -> Self{
        if self.0.iter().any(|x| x == &rhs){
            Self::new(rhs)
        } else {
            Self::EMPTY
        }
    }
}

impl<T: PartialEq, const S: char> BitXor<T> for Flags<T, S> {
    type Output = Self;
    fn bitxor(mut self, rhs: T) -> Self{
        if let Some(index) = self.0.iter().position(|x| x == &rhs){
            self.0.remove(index);
        } else {
            self.0.push(rhs)
        }
        self
    }
}

impl<T: PartialEq, const S: char> Add<Self> for Flags<T, S> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self{
        for i in rhs.into_iter(){
            if !self.0.iter().any(|x| x == &i){
                self.0.push(i)
            }
        }
        self
    }
}

impl<T: PartialEq, const S: char> Sub<Self> for Flags<T, S> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self{
        for i in rhs.into_iter(){
            if let Some(index) = self.0.iter().position(|x| x == &i){
                self.0.remove(index);
            }
        }
        self
    }
}

impl<T: PartialEq, const S: char> BitOr<Self> for Flags<T, S> {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self{
        for i in rhs.into_iter(){
            if !self.0.iter().any(|x| x == &i){
                self.0.push(i)
            }
        }
        self
    }
}

impl<T: PartialEq, const S: char> BitAnd<Self> for Flags<T, S> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self{
        Self(self.0.into_iter()
                .filter(|l| rhs.iter().any(|r| r == l))
                .collect())
    }
}

impl<T: PartialEq, const S: char> BitXor<Self> for Flags<T, S> {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self{
        for i in rhs.into_iter(){
            if let Some(pos) = self.0.iter().position(|x| x == &i) {
                self.0.remove(pos);
            } else {
                self.0.push(i);
            }
        }
        self
    }
}

impl<T: PartialEq, const S: char> AddAssign<T> for Flags<T, S> {
    fn add_assign(&mut self, rhs: T) {
        if !self.0.iter().any(|x| x == &rhs){
            self.0.push(rhs)
        }
    }
}

impl<T: PartialEq, const S: char> SubAssign<T> for Flags<T, S> {
    fn sub_assign(&mut self, rhs: T) {
        if let Some(index) = self.0.iter().position(|x| x == &rhs){
            self.0.remove(index);
        }
    }
}

impl<T: PartialEq, const S: char> BitOrAssign<T> for Flags<T, S> {
    fn bitor_assign(&mut self, rhs: T) {
        if !self.0.iter().any(|x| x == &rhs){
            self.0.push(rhs)
        }
    }
}

impl<T: PartialEq, const S: char> BitAndAssign<T> for Flags<T, S> {
    fn bitand_assign(&mut self, rhs: T) {
        if self.0.iter().any(|x| x == &rhs){
            *self = Self(SmallVec::new_const());
            self.0.push(rhs)
        } else {
            *self = Self(SmallVec::new_const());
        }
    }
}

impl<T: PartialEq, const S: char> BitXorAssign<T> for Flags<T, S> {
    fn bitxor_assign(&mut self, rhs: T) {
        if let Some(index) = self.0.iter().position(|x| x == &rhs){
            self.0.remove(index);
        } else {
            self.0.push(rhs)
        }
    }
}

impl<T: PartialEq, const S: char> AddAssign<Self> for Flags<T, S> {
    fn add_assign(&mut self, rhs: Self) {
        for i in rhs.into_iter(){
            if !self.0.iter().any(|x| x == &i){
                self.0.push(i)
            }
        }
    }
}

impl<T: PartialEq, const S: char> SubAssign<Self> for Flags<T, S> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in rhs.into_iter(){
            if let Some(index) = self.0.iter().position(|x| x == &i){
                self.0.remove(index);
            }
        }
    }
}

impl<T: PartialEq, const S: char> BitOrAssign<Self> for Flags<T, S> {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in rhs.into_iter(){
            if !self.0.iter().any(|x| x == &i){
                self.0.push(i)
            }
        }
    }
}

impl<T: PartialEq, const S: char> BitAndAssign<Self> for Flags<T, S> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(
            std::mem::take(&mut self.0)
                .into_iter()
                .filter(|l| rhs.iter().any(|r| r == l))
                .collect())
    }
}

impl<T: PartialEq, const S: char> BitXorAssign<Self> for Flags<T, S> {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in rhs.into_iter(){
            if let Some(pos) = self.0.iter().position(|x| x == &i) {
                self.0.remove(pos);
            } else {
                self.0.push(i);
            }
        }
    }
}
