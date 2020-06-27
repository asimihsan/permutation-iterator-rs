// Copyright 2019, Asim Ihsan
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

//! Utilities for iterating over random permutations.
//!
//! `permutation-iterator` provides utilities for iterating over random permutations in constant
//! space.
//!
//! # Quick Start
//!
//! Please check the GitHub repository's `README.md` and `examples` folder for how to get started
//! with this library.
#![no_std]

use core::hash::Hasher;
use wyhash::WyHash;

/// Permutor gives you back a permutation iterator that returns a random permutation over
/// [0, max) (0 inclusive to max exclusive).
///
/// # Examples
///
/// Permutor can be used to iterate over a random permutation of integers [0..max) (0 inclusive to
/// max exclusive):
///
/// ```
/// use crate::permutation_iterator::Permutor;
/// use std::collections::HashSet;
///
/// let max: u128 = 10;
/// let permutor = Permutor::new(max).expect("Expected new Permutor");
/// for value in permutor {
///     println!("{}", value);
/// }
/// ```
pub struct Permutor {
    feistel: FeistelNetwork,
    max: u128,
    current: u128,
    values_returned: u128,
}

impl Permutor {
    pub fn new_with_u64_key(max: u128, key: u64) -> anyhow::Result<Permutor> {
        let key = u64_to_32slice(key);
        Ok(Permutor {
            feistel: FeistelNetwork::new_with_slice_key(max, key),
            max,
            current: 0,
            values_returned: 0,
        })
    }

    pub fn new_with_slice_key(max: u128, key: [u8; 32]) -> anyhow::Result<Permutor> {
        Ok(Permutor {
            feistel: FeistelNetwork::new_with_slice_key(max, key),
            max,
            current: 0,
            values_returned: 0,
        })
    }

    pub fn new(max: u128) -> anyhow::Result<Permutor> {
        Ok(Permutor {
            feistel: FeistelNetwork::new(max)?,
            max,
            current: 0,
            values_returned: 0,
        })
    }
}

impl Iterator for Permutor {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        while self.values_returned < self.max {
            let next = self.feistel.permute(self.current);
            self.current += 1;
            if next >= self.max {
                continue;
            }
            self.values_returned += 1;
            return Some(next);
        }
        None
    }
}

/// Iterate over a random permutation of a pair of integer sequences.
///
/// # Examples
///
/// Suppose you have two lists, first with 3. elements and the second with 7 elements,
/// and you want to iterate over a random permutation of pairs:
///
/// ```
/// use permutation_iterator::RandomPairPermutor;
///
/// let pair_permutor = RandomPairPermutor::new(3, 7).expect("expected new RandomPairPermutor");
/// for (i, j) in pair_permutor {
///     println!("({}, {})", i, j);
/// }
/// ```
///
pub struct RandomPairPermutor {
    permutor: Permutor,
    max2: u64,
}

impl RandomPairPermutor {
    pub fn new(max1: u64, max2: u64) -> anyhow::Result<RandomPairPermutor> {
        let max: u128 = (max1 as u128) * (max2 as u128);
        Ok(RandomPairPermutor {
            permutor: Permutor::new(max)?,
            max2,
        })
    }
}

impl Iterator for RandomPairPermutor {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutor.next() {
            Some(value) => {
                let first = value as u64 / self.max2;
                let second = value as u64 % self.max2;
                Some((first, second))
            }
            _ => None,
        }
    }
}

/// Implements a Feistel network, which can take a non-invertible pseudo-random function (PRF)
/// and turn it into an invertible pseudo-random permutation (PRP).
///
/// If you use this struct directly note that its intended purpose is to be a PRP and map from
/// an n-bit input to an n-bit output, where n is an even positive integer. For example, if
/// constructed with a `max` of `10`, internally it creates a 4-bit Feistel network, and for all
/// integers in the 4-bit domain `[0, 16)` (`0` inclusive to `16` exclusive) it will map an input
/// to one and only one output, and vice-versa (a given output maps to one and only one input).
/// Even though you specified a max value of `10`, the output range may be larger than expected.
/// Clients like `RandomPermutor` handle this by excluding output values outside of the desired
/// range.
///
/// This is useful in fields like cryptography, where a block cipher is a PRP.
///
/// Another great use of a Feistel network is when you want some input to always map to one and only
/// one output (and vice versa). For example, given a 32-bit IP address, we could use some secret
/// key and map each IP address to some other 32-bit IP address. We could log this new 32-bit
/// IP address and people who do not know what the secret key is would find it difficult
/// to determine what the input IP address was. This is Format Preserving Encryption (FPE).
pub struct FeistelNetwork {
    /// TODO visible just for testing, fix
    pub half_width: u128,

    /// Mask used to keep within the width for the right.
    /// TODO visible just for testing, fix
    pub right_mask: u128,

    /// Mask used to keep within the width for the left.
    /// TODO visible just for testing, fix
    pub left_mask: u128,

    /// Private key, some random seed. 256 bits as 32 bytes.
    key: [u8; 32],

    rounds: u8,
}

impl FeistelNetwork {
    /// Create a new FeistelNetwork instance that can give you a random permutation of
    /// integers.
    ///
    /// Note that the value of max is rounded up to the nearest even power of 2. If clients are
    /// trying to get a permutation of [0, max) they need to iterate over the input range and
    /// discard values from FeistelNetwork >= max.
    ///
    /// The key used for the permutation is made up of securely gathered 32 bytes.
    pub fn new(max: u128) -> anyhow::Result<FeistelNetwork> {
        let mut key: [u8; 32] = [0; 32];
        getrandom::getrandom(&mut key).map_err(anyhow::Error::msg)?;
        Ok(FeistelNetwork::new_with_slice_key(max, key))
    }

    /// Create a new FeistelNetwork instance that can give you a random permutation of
    /// integers.
    ///
    /// Note that the value of max is rounded up to the nearest even power of 2. If clients are
    /// trying to get a permutation of [0, max) they need to iterate over the input range and
    /// discard values from FeistelNetwork >= max.
    pub fn new_with_slice_key(max_value: u128, key: [u8; 32]) -> FeistelNetwork {
        let mut width = integer_log2(max_value).unwrap();
        if width % 2 != 0 {
            width += 1;
        }
        let half_width = width / 2;
        let mut right_mask = 0;
        for i in 0..half_width {
            right_mask |= 1 << i;
        }
        let left_mask = right_mask << half_width;
        FeistelNetwork {
            half_width,
            right_mask,
            left_mask,
            key,
            rounds: 32,
        }
    }

    pub fn permute(&self, input: u128) -> u128 {
        let mut left = (input & self.left_mask) >> self.half_width;
        let mut right = input & self.right_mask;

        for i in 0..self.rounds as u8 {
            let new_left = right;
            let f = self.round_function(right, i, self.key, self.right_mask);
            right = left ^ f;
            left = new_left;
        }

        let result = (left << self.half_width) | right;
        result & (self.left_mask | self.right_mask)
    }

    fn round_function(&self, right: u128, round: u8, key: [u8; 32], mask: u128) -> u128 {
        let right_bytes = u128_to_16slice(right);
        let round_bytes = u8_to_1slice(round);

        let mut hasher = WyHash::default();
        hasher.write(&key[..]);
        hasher.write(&right_bytes[..]);
        hasher.write(&round_bytes[..]);
        hasher.write(&key[..]);
        (hasher.finish() as u128) & mask
    }
}

fn u8_to_1slice(input: u8) -> [u8; 1] {
    let mut result: [u8; 1] = [0; 1];
    result[0] = input;
    result
}

/// Convert an unsigned 128 bit number so a slice of 16 bytes in big-endian format (most significant
/// bit first).
///
/// # Examples
///
/// ```
/// use crate::permutation_iterator::u128_to_16slice;
/// let output = u128_to_16slice(42);
/// assert_eq!(output, [0, 0, 0, 0, 0, 0, 0, 0,
///                     0, 0, 0, 0, 0, 0, 0, 0x2A]);
/// ```
pub fn u128_to_16slice(input: u128) -> [u8; 16] {
    let mut result: [u8; 16] = [0; 16];
    result[15] = ((input >> 00) & 0xFF) as u8;
    result[14] = ((input >> 08) & 0xFF) as u8;
    result[13] = ((input >> 16) & 0xFF) as u8;
    result[12] = ((input >> 24) & 0xFF) as u8;
    result[11] = ((input >> 32) & 0xFF) as u8;
    result[10] = ((input >> 40) & 0xFF) as u8;
    result[09] = ((input >> 48) & 0xFF) as u8;
    result[08] = ((input >> 56) & 0xFF) as u8;
    result[07] = ((input >> 64) & 0xFF) as u8;
    result[06] = ((input >> 72) & 0xFF) as u8;
    result[05] = ((input >> 80) & 0xFF) as u8;
    result[04] = ((input >> 88) & 0xFF) as u8;
    result[03] = ((input >> 96) & 0xFF) as u8;
    result[02] = ((input >> 104) & 0xFF) as u8;
    result[01] = ((input >> 112) & 0xFF) as u8;
    result[00] = ((input >> 120) & 0xFF) as u8;
    result
}
/// Convert an unsigned 64 bit number so a slice of 8 bytes in big-endian format (most significant
/// bit first).
///
/// # Examples
///
/// ```
/// use crate::permutation_iterator::u64_to_8slice;
/// let output = u64_to_8slice(42);
/// assert_eq!(output, [0, 0, 0, 0, 0, 0, 0, 0x2A]);
/// ```
pub fn u64_to_8slice(input: u64) -> [u8; 8] {
    let mut result: [u8; 8] = [0; 8];
    result[07] = ((input >> 00) & 0xFF) as u8;
    result[06] = ((input >> 08) & 0xFF) as u8;
    result[05] = ((input >> 16) & 0xFF) as u8;
    result[04] = ((input >> 24) & 0xFF) as u8;
    result[03] = ((input >> 32) & 0xFF) as u8;
    result[02] = ((input >> 40) & 0xFF) as u8;
    result[01] = ((input >> 48) & 0xFF) as u8;
    result[00] = ((input >> 56) & 0xFF) as u8;
    result
}

/// Convert an unsigned 64 bit number so a slice of 32 bytes in big-endian format (most significant
/// bit first).
///
/// # Examples
///
/// ```
/// use crate::permutation_iterator::u64_to_32slice;
/// let output = u64_to_32slice(42);
/// assert_eq!(output, [0, 0, 0, 0, 0, 0, 0, 0x2A,
///                     0, 0, 0, 0, 0, 0, 0, 0,
///                     0, 0, 0, 0, 0, 0, 0, 0,
///                     0, 0, 0, 0, 0, 0, 0, 0]);
/// ```
pub fn u64_to_32slice(input: u64) -> [u8; 32] {
    let result8 = u64_to_8slice(input);
    let mut result: [u8; 32] = [0; 32];
    result[..8].clone_from_slice(&result8[..8]);
    result
}

/// Calculate log2 of an integer input. This tells you how many bits are required to represent the
/// input.
///
/// # Examples
///
/// ```
/// use permutation_iterator::integer_log2;
/// assert_eq!(None, integer_log2(0), "failed for {}", 0);
/// assert_eq!(Some(1), integer_log2(1), "failed for {}", 1);
/// assert_eq!(Some(2), integer_log2(2), "failed for {}", 2);
/// assert_eq!(Some(2), integer_log2(3), "failed for {}", 3);
/// assert_eq!(Some(3), integer_log2(4), "failed for {}", 4);
/// assert_eq!(Some(3), integer_log2(5), "failed for {}", 5);
/// assert_eq!(Some(3), integer_log2(6), "failed for {}", 6);
/// assert_eq!(Some(3), integer_log2(7), "failed for {}", 7);
/// assert_eq!(Some(4), integer_log2(8), "failed for {}", 8);
/// assert_eq!(Some(4), integer_log2(9), "failed for {}", 9);
/// assert_eq!(Some(4), integer_log2(10), "failed for {}", 9);
/// ```
pub fn integer_log2(input: u128) -> Option<u128> {
    if input == 0 {
        return None;
    }
    let mut result = 0;
    let mut input_copy = input;
    while input_copy > 0 {
        input_copy >>= 1;
        result += 1;
    }
    Some(result)
}
