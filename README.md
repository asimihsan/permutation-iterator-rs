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
permutation_iterator = "0.1.1"
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

One way of generating a random permutation is to shuffle a list. For example, given input integers `[0, 1, 2, 3, 4, 5]`,
one can shuffle it to e.g. `[5, 3, 2, 0, 1, 4]`. Each input element maps to one and only one output element, and
vice versa (each output element maps to one and only one input element). As you consume the shuffled list from e.g.
left to right you're consuming this random permutation.

Shuffling is `O(n)` time using the Fisher-Yates algorithm, however it is also `O(n)` space. We need a copy of the
elements in-memory in order to shuffle them. This is inconvenient if the input range is large, or if the environment
you're running on is memory-constrained.

Cryptography offers an alternative. Symmetric encryption boils down to mapping a given input to one and only one output,
where the mapping is varied by a single secret key, and vice-versa (each output element maps to one and only one input
element). If this **bijective** mapping did not exist we wouldn't be reliably able to retrieve the original input. One
specific kind of symmetric encryption uses a **block** cipher (operating on n-bits at a time) implemented using a 
**Feistel network**.

A Feistel network is an extraordinary construct that allows you to use a simple, relatively weak **non-invertible**
function over and over again and become a complicated, relatively strong **invertible permutation**. Hence in
constant time we can _encrypt_ inputs as a way of iterating over random permutations. We can similarly _decrypt_
the output as a way of _resuming_ permutations.

Consider the example of a bank that is trying to generate unique credit card numbers. Actual credit card numbers need
to be stored very securely and we would rather not have to look them in order to find the next available number. By
storing just a key and the last credit card number generated we can securely and efficiently continue iterating over
the random permutation of all credit card numbers, without risking repeats.

## License

`permutation-iterator` is distributed under the terms of the Apache License (Version 2.0). See [LICENSE](LICENSE) for
details.
