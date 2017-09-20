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
//!     let mut capped_set = CappedMultiset::new(None);
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

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;


/// A `CappedMultiset` structure is a data structure similar to a multiset with the key distinction
/// that it supports setting a _cap_ on the values of each element. Once a cap is set, all
/// operations on the data structure that access an element will return at most the value of the
/// cap.
#[derive(Hash, Debug, Clone)]
pub struct CappedMultiset<U> {
    /// A vector containing all the elements in the multiset in their original form.
    elements: BTreeMap<U, usize>,
    /// The cap that is applied to the elements. This is an artificial cap since it does not modify
    /// the actual value stored, but only the one displayed through various operations.
    cap: Option<usize>,
}

impl<U> CappedMultiset<U>
where
    U: Ord,
{
    /// Creates a new `CappedMultiset` along with a cap. Pass `None` as the
    /// value to prevent any caps from being set.
    ///
    /// # Example
    ///
    /// ```
    /// use capped_multiset::CappedMultiset;
    ///
    /// let mset: CappedMultiset<u32> = CappedMultiset::new(None);
    /// ```
    pub fn new(cap: Option<usize>) -> Self {
        CappedMultiset {
            elements: BTreeMap::new(),
            cap: cap,
        }
    }

    /// Inserts an element into the Multiset.
    /// This operation always succeeds irrespective of the current value of the
    /// `cap`.
    ///
    /// # Example
    /// ```
    /// use capped_multiset::CappedMultiset;
    ///
    /// let mut mset: CappedMultiset<u32> = CappedMultiset::new(None);
    /// mset.insert(5);
    /// assert_eq!(1, mset.count_of(5));
    /// ```
    pub fn insert(&mut self, val: U) {
        self.insert_multiple(val, 1);
    }

    /// Inserts an element (`E`) multiple (`n`) times into the Multiset
    /// This operation will always succeed and add the element to the Multiset
    /// irrespective of the current value of the `cap`.
    ///
    /// # Example
    /// ```
    /// use capped_multiset::CappedMultiset;
    ///
    /// let mut mset: CappedMultiset<u32> = CappedMultiset::new(Some(5));
    /// mset.insert_multiple(3, 2);
    /// mset.insert_multiple(4, 8);
    /// assert_eq!(2, mset.count_of(3));
    /// assert_eq!(5, mset.count_of(4));
    /// ```
    pub fn insert_multiple(&mut self, elem: U, n: usize) {
        match self.elements.entry(elem) {
            Entry::Vacant(view) => {
                view.insert(n);
            }
            Entry::Occupied(mut view) => {
                *(view.get_mut()) += n;
            }
        };
    }

    /// Set the cap for the Multiset elements.
    ///
    /// This is *not* a lossy operation. Setting a lower cap will not modify
    /// the data in the Multiset at all. Hence, setting the cap is a cheap O(1)
    /// operation. All future operations will however honor the new value of
    /// the cap
    ///
    /// # Example
    /// ```
    /// use capped_multiset::CappedMultiset;
    ///
    /// let mut mset: CappedMultiset<u32> = CappedMultiset::new(None);
    /// mset.insert_multiple(0, 10);
    /// assert_eq!(10, mset.count_of(0));
    /// mset.set_cap(Some(5));
    /// assert_eq!(5, mset.count_of(0));
    /// mset.set_cap(Some(15));
    /// assert_eq!(10, mset.count_of(0));
    /// ```
    pub fn set_cap(&mut self, cap: Option<usize>) {
        self.cap = cap;
    }

    /// Return the number of times Element occurs in Multiset.
    /// This method honors the current value of the `cap` and hence has an
    /// upper bound of the current value of `cap`.
    ///
    /// # Example
    /// ```
    /// use capped_multiset::CappedMultiset;
    ///
    /// let mut mset: CappedMultiset<u32> = CappedMultiset::new(Some(5));
    /// mset.insert_multiple(0, 7);
    /// mset.insert_multiple(1, 4);
    /// mset.insert(0);
    /// assert_eq!(5, mset.count_of(0));
    /// assert_eq!(4, mset.count_of(1));
    /// mset.set_cap(None);
    /// assert_eq!(8, mset.count_of(0));
    /// ```
    pub fn count_of(&self, elem: U) -> usize {
        let count = self.elements.get(&elem).map_or(0, |x| *x);
        self.capped_val(count)
    }

    /// Return a value after honoring the current `cap`
    #[inline]
    fn capped_val(&self, value: usize) -> usize {
        match self.cap {
            None => value,
            Some(c) => std::cmp::min(value, c),
        }
    }
}
