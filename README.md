# permutation-iterator

[![Build Status](https://travis-ci.com/asimihsan/permutation-iterator-rs.svg?branch=master)](https://travis-ci.com/asimihsan/permutation-iterator-rs)
[![Crate](https://img.shields.io/crates/v/permutation_iterator.svg)](https://crates.io/crates/permutation_iterator)
[![API](https://docs.rs/permutation_iterator/badge.svg)](https://docs.rs/permutation_iterator)

A Rust library for iterating over random permutations without fully materializing them into memory.

`permutation-iterator` lets you iterate over a random permutation, for example the values `[0, 1, 2, 3, 4, 5]` in a
random order. It does so in constant space; it does not fully instantiate the values in memory then shuffle them.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
permutation_iterator = "0.1.0"
```


# License

`permutation-iterator` is distributed under the terms of the Apache License (Version 2.0). See [LICENSE](LICENSE) for
details.
