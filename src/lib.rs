// #![warn(cast_possible_truncation)]
// #![warn(cast_possible_wrap)]
// #![warn(cast_sign_loss)]
// #![warn(filter_map)]
// #![warn(if_not_else)]
// #![warn(items_after_statements)]
// #![warn(nonminimal_bool)]
// #![warn(option_map_unwrap_or)]
// #![warn(option_map_unwrap_or_else)]
// #![warn(option_unwrap_used)]
// #![warn(shadow_reuse)]
// #![warn(shadow_same)]
// #![warn(shadow_unrelated)]
// #![warn(single_match_else)]
// #![warn(wrong_pub_self_convention)]

//! A multiset is a datastructure which resembles a classic Set, except it allows duplicate
//! elements for each key. For more information on Multisets, see:
//!
//! * [Wikipedia Entry](https://en.wikipedia.org/wiki/Multiset)
//! * [C++ Multisets](http://en.cppreference.com/w/cpp/container/multiset)
//! * [C++ Multiset Tutorial](http://www.java2s.com/Tutorial/Cpp/0380__set-multiset/Catalog0380__set-multiset.htm)
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

/// A `CappedMultiset` structure is a data structure similar to a multiset with the key distinction
/// that it supports setting a _cap_ on the values of each element. Once a cap is set, all
/// operations on the data structure that access an element will return at most the value of the
/// cap.
#[derive(Hash, Debug, Eq, PartialEq)]
pub struct CappedMultiset {
    elements: Vec<u32>,
    cap: u32,
}

impl CappedMultiset {
    /// Consumes a `Vec<u32>` and returns a `CappedMultiset` with the same values.
    /// By default, no cap is set on the elements of the multiset
    pub fn new(item: Vec<u32>) -> CappedMultiset {
        CappedMultiset {
            elements: item,
            cap: u32::max_value()
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

#[cfg(test)]
mod tests {
    use CappedMultiset;
    #[test]
    fn test_sum() {
        let simple_array: Vec<u32> = vec![1,2,3,4,5];
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
}
