#![feature(test)]

extern crate test;

#[cfg(test)]
mod benches {
    use permutation_iterator::FeistelNetwork;
    use test::Bencher;

    const ZERO_KEY: [u8; 32] = [0; 32];

    #[bench]
    fn bench_medium(b: &mut Bencher) {
        let feistel = FeistelNetwork::new_with_slice_key(100_000, ZERO_KEY);
        b.iter(|| feistel.permute(50_000));
    }
}
