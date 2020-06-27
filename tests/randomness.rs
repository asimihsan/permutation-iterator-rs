#[cfg(test)]
mod tests {
    use permutation_iterator::Permutor;
    use rand::prelude::StdRng;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::collections::HashMap;

    /// Given a Permutor instance, for a given maximum value over which we're looking for permutations
    /// [0, max), see if each given returned value is evenly distributed.
    ///
    /// Only run this test in release mode, or else it will take too long.
    ///
    /// Test isn't very reliable especially for small max_value's but better than nothing.
    #[test]
    #[ignore]
    fn test_randomness_g_test() {
        for (max_value, max_key, ratio_diff_threshold) in vec![
            // The true random chi-squared value is very variable, [1, 15] almost, so testing very
            // small max_value permutations reliably is difficult.
            // (4, 100_000),

            // Small-value tests are still pretty unreliable...can't really leave it in!
            (10, 200_000, 0.3),
            (17, 100_000, 0.2),
            (50, 50_000, 0.05),
            (100, 50_000, 0.05),
            (1000, 50_000, 0.05),
        ] {
            let g_test_permutor = randomness_g_test(max_value, max_key, false);
            let g_test_true_random = randomness_g_test(max_value, max_key, true);
            let ratio_diff = (g_test_permutor - g_test_true_random) / g_test_permutor;
            println!(
                "max_value: {}, max_key: {}, g_test_permutor: {:.2}, g_test_true_random: {:.2}, ratio_diff: {:.2}",
                max_value, max_key, g_test_permutor, g_test_true_random, ratio_diff
                );

            // If ratio_diff is negative, permutor is "more random" than true randomness (which is
            // absurd, just a test artifact).
            assert!(
                ratio_diff < ratio_diff_threshold,
                "Expected permutor to be as random or worse by {:.2} than true randomness!",
                ratio_diff_threshold
            );
        }
    }

    /// Reference: https://en.wikipedia.org/wiki/G-test
    fn randomness_g_test(max_value: u128, max_key: u64, true_random: bool) -> f64 {
        let mut rng: StdRng = rand::SeedableRng::seed_from_u64(42);
        let min_key = 0;
        let mut cell_counts: HashMap<usize, HashMap<u128, u32>> =
            HashMap::with_capacity(max_value as usize);
        for key in min_key..max_key {
            let values: Vec<u128>;
            if true_random {
                values = get_random_permutation_true_random(max_value, &mut rng);
            } else {
                values = get_random_permutation_permutor(max_value, key);
            }
            for (i, value) in values.into_iter().enumerate() {
                if !cell_counts.contains_key(&i) {
                    cell_counts.insert(i, HashMap::with_capacity(max_value as usize));
                }
                let cell_count = cell_counts.get_mut(&i).unwrap();
                let current_value = cell_count.get(&value).cloned().unwrap_or(0);
                cell_count.insert(value, current_value + 1);
            }
        }

        let mut g_test_sum: f64 = 0.0;
        let expected_count: f64 = (max_key - min_key) as f64 / max_value as f64;
        for (_cell_index, cell_count) in cell_counts {
            for (_cell_value, observed_count) in cell_count {
                let g_test_subvalue =
                    (observed_count as f64 / expected_count).ln() * observed_count as f64;
                g_test_sum += g_test_subvalue;
            }
        }
        2.0 * g_test_sum
    }

    fn get_random_permutation_permutor(max_value: u128, key: u64) -> Vec<u128> {
        let permutor = Permutor::new_with_u64_key(max_value, key).expect("expected new Permutor");
        permutor.collect()
    }

    fn get_random_permutation_true_random(max_value: u128, rng: &mut impl Rng) -> Vec<u128> {
        let mut values: Vec<u128> = (0..max_value).collect();
        values.shuffle(rng);
        values
    }
}
