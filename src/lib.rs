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

use blake2_rfc::blake2b::Blake2b;
use rand::Rng;

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
/// let max: u64 = 10;
/// let permutor = Permutor::new(max);
/// for value in permutor {
///     println!("{}", value);
/// }
/// ```
pub struct Permutor {
    feistel: FeistelNetwork,
    max: u64,
    current: u64,
    values_returned: u64,
}

impl Permutor {
    pub fn new_with_u64_key(max: u64, key: u64) -> Permutor {
        let key = u64_to_32slice(key);
        Permutor {
            feistel: FeistelNetwork::new_with_slice_key(max, key),
            max,
            current: 0,
            values_returned: 0,
        }
    }

    pub fn new_with_slice_key(max: u64, key: [u8; 32]) -> Permutor {
        Permutor {
            feistel: FeistelNetwork::new_with_slice_key(max, key),
            max,
            current: 0,
            values_returned: 0,
        }
    }

    pub fn new(max: u64) -> Permutor {
        Permutor {
            feistel: FeistelNetwork::new(max),
            max,
            current: 0,
            values_returned: 0,
        }
    }
}

impl Iterator for Permutor {
    type Item = u64;

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
        return None;
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
/// let pair_permutor = RandomPairPermutor::new(3, 7);
/// for (i, j) in pair_permutor {
///     println!("({}, {})", i, j);
/// }
/// ```
///
pub struct RandomPairPermutor {
    permutor: Permutor,
    max2: u32,
}

impl RandomPairPermutor {
    pub fn new(max1: u32, max2: u32) -> RandomPairPermutor {
        let max: u64 = (max1 as u64) * (max2 as u64);
        RandomPairPermutor {
            permutor: Permutor::new(max),
            max2,
        }
    }
}

impl Iterator for RandomPairPermutor {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutor.next() {
            Some(value) => {
                let first = value as u32 / self.max2;
                let second = value as u32 % self.max2;
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
    pub half_width: u64,

    /// Mask used to keep within the width for the right.
    /// TODO visible just for testing, fix
    pub right_mask: u64,

    /// Mask used to keep within the width for the left.
    /// TODO visible just for testing, fix
    pub left_mask: u64,

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
    pub fn new(max: u64) -> FeistelNetwork {
        let key = rand::thread_rng().gen::<[u8; 32]>();
        FeistelNetwork::new_with_slice_key(max, key)
    }

    /// Create a new FeistelNetwork instance that can give you a random permutation of
    /// integers.
    ///
    /// Note that the value of max is rounded up to the nearest even power of 2. If clients are
    /// trying to get a permutation of [0, max) they need to iterate over the input range and
    /// discard values from FeistelNetwork >= max.
    pub fn new_with_slice_key(max_value: u64, key: [u8; 32]) -> FeistelNetwork {
        let width = (max_value as f64).log2();
        let mut width = width.ceil() as u64;
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
            rounds: 12,
        }
    }

    pub fn permute(&self, input: u64) -> u64 {
        let mut left = (input & self.left_mask) >> self.half_width;
        let mut right = input & self.right_mask;

        for i in 0..self.rounds as u8 {
            let new_left = right;
            let f = self.round_function(right, i, &self.key[..], self.right_mask);
            right = left ^ f;
            left = new_left;
        }

        let result = (left << self.half_width) | right;
        let result = result & (self.left_mask | self.right_mask);
        result
    }

    fn round_function(&self, right: u64, round: u8, key: &[u8], mask: u64) -> u64 {
        let right_bytes = u64_to_8slice(right);
        let round_bytes = u8_to_1slice(round);
        let mut context: Blake2b = Blake2b::with_key(8, key);
        context.update(&right_bytes[..]);
        context.update(&round_bytes[..]);
        let hash = context.finalize();
        let hash_bytes: &[u8] = hash.as_bytes();
        slice_to_u64(hash_bytes) & mask
    }
}

fn slice_to_u64(input: &[u8]) -> u64 {
    ((input[7] as u64) << 0)
        | ((input[6] as u64) << 8)
        | ((input[5] as u64) << 16)
        | ((input[4] as u64) << 24)
        | ((input[3] as u64) << 32)
        | ((input[2] as u64) << 40)
        | ((input[1] as u64) << 48)
        | ((input[0] as u64) << 56)
}

fn u8_to_1slice(input: u8) -> [u8; 1] {
    let mut result: [u8; 1] = [0; 1];
    result[0] = input;
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
    result[7] = ((input & 0xFF) >> 0) as u8;
    result[6] = ((input & 0xFF00) >> 8) as u8;
    result[5] = ((input & 0xFF00_00) >> 16) as u8;
    result[4] = ((input & 0xFF00_0000) >> 24) as u8;
    result[3] = ((input & 0xFF00_0000_00) >> 32) as u8;
    result[2] = ((input & 0xFF00_0000_0000) >> 40) as u8;
    result[1] = ((input & 0xFF00_0000_0000_00) >> 48) as u8;
    result[0] = ((input & 0xFF00_0000_0000_0000) >> 56) as u8;
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
    for i in 0..8 {
        result[i] = result8[i];
    }
    result
}
