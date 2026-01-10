use std::{fmt::Debug, iter::Peekable, ops::Deref};

// use crate::segment::token::tstring::TString;

use super::utilities::split_on_first;

pub fn split_on<'a, A: Eq + Debug>(arr: &'a [A], token: &[A]) -> Vec<&'a [A]> {
    let mut result = Vec::new();

    let mut tmp = arr;

    loop {
        let split_on_first = split_on_first(tmp, token);
        match split_on_first {
            None => {
                //println!("I reached the end on: {:?}", tmp);
                result.push(tmp);
                break;
            }
            Some((pop, rest)) => {
                result.push(pop);
                tmp = rest; // This tmp is not the same as the tmp outside the loop, a problem
                            // I need to look at
            }
        }
    }

    result
}

pub trait Splittable<T> {
    fn split_on_first<'a>(self, token: T) -> Option<(T, T)>;
    // fn split_on_first_general<'a, F>(self, pred: &F) -> Option<(T, T)>
    // where F: Fn(&Self) -> bool;
}

// impl<'a, T: Splittable<T>> Splittable<&'a mut T> for &'a mut T {
//
//     fn split_on_first(self, token: Self) -> Option<(Self, Self)> {
//             self.split_on_first(token)
//     }
// }

/*
!!!!!IMPORTANT!!!!!!
//In Splittable, replace the token with a predicate, (a function pointer &Fn()->bool u dumdum TwT XD)
*/
impl<T: Splittable<T>> Splittable<T> for Option<T> {
    fn split_on_first(self, token: T) -> Option<(T, T)> {
        match self {
            Some(t) => t.split_on_first(token),
            None => None,
        }
    }

    // fn split_on_first_general<'a, F>(self, pred: &F) -> Option<(T, T)>
    // where F: Fn(&T) -> bool {
    //     match self {
    //         Some(t) => {
    //             t.split_on_first_general(
    //                 &|x| pred(x)
    //             )
    //         },
    //         None => None,
    //     }
    // }
}

impl<'a, T: Eq + Debug> Splittable<&'a [T]> for &'a [T] {
    fn split_on_first(self, token: &[T]) -> Option<(&'a [T], &'a [T])> {
        split_on_first(self, token)
    }

    // fn split_on_first_general<F>(self, pred: &F) -> Option<(&'a [T], &'a [T])>
    // where F: Fn(&Self) -> bool {
    //     todo!()
    // }
}

impl<'a, T: Eq + Debug> Splittable<&'a [T]> for &'a mut [T] {
    fn split_on_first(self, token: &[T]) -> Option<(&'a [T], &'a [T])> {
        split_on_first(self, token)
    }

    // fn split_on_first_general<F>(self, pred: &F) -> Option<(&'a [T], &'a [T])>
    // where F: Fn(&Self) -> bool {
    //     todo!()
    // }
}

#[derive(Debug)]
pub struct SplitOn<T: Eq + Debug + Splittable<T>> {
    subobject: T,
    //next: Option<&'a T>,
    object: T,
}

impl<T: Eq + Copy + Debug + Splittable<T>> SplitOn<T> {
    pub fn new(arr: T, token: T) -> Self {
        SplitOn {
            object: arr,
            subobject: token,
            //next: None
        }
    }

    //pub fn is_empty(&self) -> bool {
    //    self.object == T::default()
    //}

    pub fn peek(&self) -> Option<T> {
        let (pre, _post) = self.object.split_on_first(self.subobject)?;
        Some(pre)
    }

    //pub(crate) fn map(&self, to_vec: impl Fn(_) -> TString) -> _ {
    //    todo!()
    //}
}

impl<'a, T: Eq + Debug> Iterator for SplitOn<&'a [T]> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        match split_on_first(self.object, self.subobject) {
            Some((pre, post)) => {
                self.object = post;
                Some(pre)
            }
            None => {
                if self.object.is_empty() {
                    None
                } else {
                    let result = Some(self.object);
                    self.object = &[];
                    result
                }
            }
        }
    }

    // fn next(&mut self) -> Option<Self::Item> {
    //     let (pre, post) = split_on_first(self.object, self.subobject)?;
    //     self.object = post;
    //     Some(pre)
    // }
}

// impl Iterator for SplitOn<TString> {
//     type Item = &'a [TokenString];
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let (next, _rest) = self.object.split_on_first(self.subobject)?;
//         Some(next)
//     }
// }

// impl<'a, T: Splittable<T>> Iterator for SplitOn<&'a T>
// where   T: Eq + Deref + Debug + Sized + Copy, <T as Deref>::Target: Sized + Copy {
//     type Item = &'a <T as Deref>::Target;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        let (next, rest) = self.object.split_on_first(self.subobject)?;
//        self.object = rest;
//        Some(next)
//    }
// }
