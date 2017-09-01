//! A multiset is a datastructure which resembles a classic Set, except it allows duplicate
//! elements for each key. For more information on Multisets, see:
//!
//! * [Wikipedia Entry](https://en.wikipedia.org/wiki/Multiset)
//! * [C++ Multisets](http://en.cppreference.com/w/cpp/container/multiset)
//! * [Tut](http://www.java2s.com/Tutorial/Cpp/0380__set-multiset/Catalog0380__set-multiset.htm)
//! * _Knuth, Donald. The Art of Computer Programming Volume II, Section 4.6.3, Exercise 19_
//!
//! This crate implements a `CappedMultiset`. A `CappedMultiset` is a datastructure similar to a
//! multiset, except it can have a dynamically defined "cap" on the value of each key. When such a
//! cap is defined, any operation to retrieve the value of an element of the set, the value
//! returned will be no greater than the "cap" on the multiset. This `cap` can be changed at
//! runtime and does not affect the actual data stored in the Multiset. As a result, setting
//! `cap = 1` or any other low value is not a lossy operation.
//!
//! ```rust
//! extern crate capped_multiset;
//!
//! use capped_multiset::CappedMultiset;
//!
//! fn main() {
//!     let set = vec![1, 2, 3, 4, 5];
//!     let mut capped_set = CappedMultiset::new(set);
//!     assert_eq!(capped_set.sum(), 15);
//!     capped_set.set_cap(Some(1));
//!     assert_eq!(capped_set.sum(), 5);
//!     capped_set.set_cap(Some(2));
//!     assert_eq!(capped_set.sum(), 9);
//! }
//! ```

// Clippy Lints
#![allow(unknown_lints)]
#![warn(cast_possible_truncation)]
#![warn(cast_possible_wrap)]
#![warn(cast_precision_loss)]
#![warn(cast_sign_loss)]
#![warn(empty_enum)]
#![warn(enum_glob_use)]
#![warn(filter_map)]
#![warn(if_not_else)]
#![warn(indexing_slicing)]
#![warn(invalid_upcast_comparisons)]
#![warn(items_after_statements)]
#![warn(missing_docs_in_private_items)]
#![warn(mut_mut)]
#![warn(nonminimal_bool)]
#![warn(option_map_unwrap_or)]
#![warn(option_map_unwrap_or_else)]
#![warn(option_unwrap_used)]
#![warn(pub_enum_variant_names)]
#![warn(result_unwrap_used)]
#![warn(shadow_reuse)]
#![warn(shadow_same)]
#![warn(shadow_unrelated)]
#![warn(similar_names)]
#![warn(single_match_else)]
#![warn(stutter)]
#![warn(wrong_pub_self_convention)]

#![warn(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]


use std::ops::{BitOrAssign, BitOr, BitAnd, BitAndAssign};

/// A `CappedMultiset` structure is a data structure similar to a multiset with the key distinction
/// that it supports setting a _cap_ on the values of each element. Once a cap is set, all
/// operations on the data structure that access an element will return at most the value of the
/// cap.
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub struct CappedMultiset {
    /// A vector containing all the elements in the multiset in their original form.
    elements: Vec<u32>,
    /// The cap that is applied to the elements. This is an artificial cap since it does not modify
    /// the actual value stored, but only the one displayed through various operations.
    cap: u32,
}

/// Convert from a slice into a CappedMultiset
impl<'a> From<&'a [u32]> for CappedMultiset {
    fn from(slice: &[u32]) -> Self {
        let vec = slice.to_owned();
        CappedMultiset::new(vec)
    }
}

/// Convert a vector into a CappedMultiset
impl From<Vec<u32>> for CappedMultiset {
    fn from (vec: Vec<u32>) -> Self {
        CappedMultiset::new(vec)
    }
}

impl CappedMultiset {
    /// Consumes a `Vec<u32>` and returns a `CappedMultiset` with the same values.
    /// By default, no cap is set on the elements of the multiset
    pub fn new(item: Vec<u32>) -> CappedMultiset {
        CappedMultiset {
            elements: item,
            cap: u32::max_value(),
        }
    }

    /// Compute the sum of all elements of the multiset, honoring the value of the cap.
    pub fn sum(&self) -> u32 {
        let mut sum = 0;
        for elem in self.elements.iter().map(|&x| std::cmp::min(x, self.cap)) {
            sum += elem;
        }
        sum
    }

    /// Set a cap on the values of the multiset
    pub fn set_cap(&mut self, cap: Option<u32>) {
        self.cap = cap.unwrap_or(u32::max_value());
    }
}

impl BitAndAssign for CappedMultiset {
    /// In-place intersection of the `CappedMultiset` and `_rhs`
    ///
    /// Compares LHS and RHS element-wise and stores the minimum for each element in LHS
    fn bitand_assign(&mut self, _rhs: CappedMultiset) {
        for (e1, e2) in self.elements.iter_mut().zip(_rhs.elements.iter()) {
            *e1 = std::cmp::min(*e1, *e2);
        }
    }
}

impl<'a> BitAndAssign<&'a CappedMultiset> for CappedMultiset {
    /// In-place intersection of the `CappedMultiset` and a reference to `_rhs`
    fn bitand_assign(&mut self, _rhs: &'a CappedMultiset) {
        for (e1, e2) in self.elements.iter_mut().zip(_rhs.elements.iter()) {
            *e1 = std::cmp::min(*e1, *e2);
        }
    }
}

impl BitAnd for CappedMultiset {
    type Output = Self;

    /// Returns the intersection of self and rhs as a new CappedMultiset.
    ///
    /// Compares LHS and RHS element-wise and returns a new `CappedMultiset` containing the minimum
    /// for each element
    fn bitand(self, rhs: Self) -> Self {
        let mut result = CappedMultiset::new(self.elements);
        result &= rhs;
        result
    }
}

impl BitOrAssign for CappedMultiset {
    /// In-place union of the `CappedMultiset` and `_rhs`
    fn bitor_assign(&mut self, _rhs: CappedMultiset) {
        for (e1, e2) in self.elements.iter_mut().zip(_rhs.elements.iter()) {
            *e1 = std::cmp::max(*e1, *e2);
        }
    }
}

impl<'a> BitOrAssign<&'a CappedMultiset> for CappedMultiset {
    fn bitor_assign(&mut self, _rhs: &'a CappedMultiset) {
        for (e1, e2) in self.elements.iter_mut().zip(_rhs.elements.iter()) {
            *e1 = std::cmp::max(*e1, *e2);
        }
    }
}

impl BitOr for CappedMultiset {
    type Output = Self;

    /// Returns the union of self and rhs as a new CappedMultiset.
    fn bitor(self, rhs: Self) -> Self {
        let mut result = CappedMultiset::new(self.elements);
        result |= rhs;
        result
    }
}


#[cfg(test)]
mod tests {
    use CappedMultiset;
    #[test]
    fn test_sum() {
        let simple_array: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut testset = CappedMultiset::new(simple_array);
        assert_eq!(testset.sum(), 15);
        testset.set_cap(Some(3));
        assert_eq!(testset.sum(), 12);
        testset.set_cap(None);
        assert_eq!(testset.sum(), 15);
        testset.set_cap(Some(1));
        assert_eq!(testset.sum(), 5);
        testset.set_cap(Some(0));
        assert_eq!(testset.sum(), 0);
    }

    #[test]
    fn test_operations() {
        let set1_vec: Vec<u32> = vec![2, 4, 6, 8, 10];
        let set2_vec: Vec<u32> = vec![2, 3, 4, 10, 12];
        let testset1 = CappedMultiset::new(set1_vec);
        let testset2 = CappedMultiset::new(set2_vec);
        let testset3 = testset1.clone() | testset2.clone();
        let testset4 = testset1.clone() & testset2.clone();
        assert_eq!(testset3.sum(), 34);
        assert_eq!(testset4.sum(), 27);
    }
}
