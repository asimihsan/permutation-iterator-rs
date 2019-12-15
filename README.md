# permutation-iterator

[![Build Status](https://travis-ci.com/asimihsan/permutation-iterator-rs.svg?branch=master)](https://travis-ci.com/asimihsan/permutation-iterator-rs)

A Rust library for iterating over random permutations without fully materializing them into memory.

`permutation-iterator` lets you iterate over a random permutation, for example the values `[0, 1, 2, 3, 4, 5]` in a
random order. It does so in constant space; it does not fully instantiate the values in memory then shuffle them.
