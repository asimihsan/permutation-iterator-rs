# permutation-iterator

[![Build Status](https://travis-ci.com/asimihsan/permutation-iterator-rs.svg?branch=master)](https://travis-ci.com/asimihsan/permutation-iterator-rs)
[![Crate](https://img.shields.io/crates/v/permutation_iterator.svg)](https://crates.io/crates/permutation_iterator)
[![API](https://docs.rs/permutation_iterator/badge.svg)](https://docs.rs/permutation_iterator)
![License](https://img.shields.io/crates/l/permutation_iterator.svg)


A Rust library for iterating over random permutations without fully materializing them into memory.

`permutation-iterator` lets you iterate over a random permutation, for example the values `[0, 1, 2, 3, 4, 5]` in a
random order. It does so in constant space; it does not fully instantiate the values in memory then shuffle them.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
permutation_iterator = "0.1.0"
```

## Example

### Random, single integer range

Here is how to iterate over a random permutation of integers in the range `[0, max)`, i.e. `0` inclusive to `max`
exclusive. Every time you run this you will get a different permutation.

```rust
use permutation_iterator::Permutor;

fn main() {
    let max = 10;
    let permutor = Permutor::new(max);
    for permuted in permutor {
        println!("{}", permuted);
    }
}
```

### Deterministic, single integer range

You can also pass in a `key` in order to iterate over a deterministically random permutation. Every time you run this
you will get the same permutation:

```rust
use permutation_iterator::Permutor;

fn main() {
    let max = 10;
    let key: [u8; 32] = [0xBA; 32];
    let permutor = Permutor::new_with_slice_key(max, key);
    for permuted in permutor {
        println!("{}", permuted);
    }
}
```

### Random, pair of integers

If you have e.g. two vectors of integers and you want to iterate over a random permutation of pairs from these lists
you can use:

```rust
use permutation_iterator::RandomPairPermutor;

fn main() {
    let xs = [1, 2, 3];
    let ys = [4, 5, 6, 7, 8];

    let permutor = RandomPairPermutor::new(xs.len() as u32, ys.len() as u32);
    for (i, j) in permutor {
        println!("({}, {})", xs[i as usize], ys[j as usize]);
    }
}
```

## Implementation details



## License

`permutation-iterator` is distributed under the terms of the Apache License (Version 2.0). See [LICENSE](LICENSE) for
details.
