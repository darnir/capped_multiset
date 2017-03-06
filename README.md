[![Build Status](https://travis-ci.org/darnir/capped_multiset.svg?branch=master)](https://travis-ci.org/darnir/capped_multiset)
[![Crates.io](https://img.shields.io/crates/v/capped_multiset.svg)](https://crates.io/crates/capped_multiset)
[![Crates.io](https://img.shields.io/crates/l/capped_multiset.svg)](./LICENSE)
[![Docs.rs](https://docs.rs/capped_multiset/badge.svg)](https://docs.rs/capped_multiset)

# CappedMultiset

A multiset is a datastructure which resembles a classic Set, except it allows duplicate
elements for each key. For more information on Multisets, see:

* [Wikipedia Entry](https://en.wikipedia.org/wiki/Multiset)
* [C++ Multisets](http://en.cppreference.com/w/cpp/container/multiset)
* [C++ Multiset Tutorial](http://www.java2s.com/Tutorial/Cpp/0380__set-multiset/Catalog0380__set-multiset.htm)
* _Knuth, Donald. The Art of Computer Programming Volume II, Section 4.6.3, Exercise 19_

This crate implements a `CappedMultiset`. A `CappedMultiset` is a datastructure similar to a
multiset, except it can have a dynamically defined "cap" on the value of each key. When such a
cap is defined, any operation to retrieve the value of an element of the set, the value
returned will be no greater than the "cap" on the multiset. This `cap` can be changed at
runtime and does not affect the actual data stored in the Multiset. As a result, setting
`cap = 1` or any other low value is not a lossy operation.

## Installation

```toml
[dependencies]
capped_multiset = "0.1"
```

## Example

```rust
extern crate capped_multiset;

use capped_multiset::CappedMultiset;

fn main() {
    let set = vec![1, 2, 3, 4, 5];
    let mut capped_set = CappedMultiset::new(set);
    assert_eq!(capped_set.sum(), 15);
    capped_set.set_cap(Some(1));
    assert_eq!(capped_set.sum(), 5);
    capped_set.set_cap(Some(2));
    assert_eq!(capped_set.sum(), 9);
}
```

## License
MIT
