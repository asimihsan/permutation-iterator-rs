#[cfg(test)]
mod tests {
    use permutation_iterator::Permutor;
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
    fn test_randomness_chi_squared() {
        for (max_value, max_key, ratio_diff_threshold) in vec![
            // The true random chi-squared value is very variable, [1, 15] almost, so testing very
            // small max_value permutations reliably is difficult.
            // (4, 100_000),

            // Small-value tests are still pretty unreliable...so need bigger diff.
            (10, 60_000, 0.3),
            (17, 24_000, 0.2),
            (50, 8_000, 0.05),
            (100, 4_000, 0.05),
            (1000, 1_000, 0.05),
        ] {
            let chi_squared_permutor = randomness_chi_squared(max_value, max_key, false);
            let chi_squared_true_random = randomness_chi_squared(max_value, max_key, true);
            let ratio_diff =
                (chi_squared_permutor - chi_squared_true_random) / chi_squared_permutor;
            println!(
                "max_value: {}, max_key: {}, chi_squared_permutor: {:.2}, chi_squared_true_random: {:.2}, ratio_diff: {:.2}",
                max_value, max_key, chi_squared_permutor, chi_squared_true_random, ratio_diff
            );

            // If ratio_diff is negative, permutor is "more random" than true randomness (which is
            // absurd, just a test artifact). We fail the test if we're "less random" by 10%.
            assert!(
                ratio_diff < ratio_diff_threshold,
                "Expected permutor to be as random or worse by {:.2} than true randomness!",
                ratio_diff_threshold
            );
        }
    }

    fn randomness_chi_squared(max_value: u64, max_key: u64, true_random: bool) -> f64 {
        let mut rng = rand::thread_rng();
        let min_key = 0;
        let mut cell_counts: HashMap<usize, HashMap<u64, u32>> =
            HashMap::with_capacity(max_value as usize);
        for key in min_key..max_key {
            let values: Vec<u64>;
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

        let expected_frequency: u32 = ((max_key - min_key) / max_value) as u32;
        let mut chi_squared_numerator: u32 = 0;
        for (_cell_index, cell_count) in cell_counts {
            for (_cell_value, count) in cell_count {
                let diff = count - expected_frequency;
                let diff = diff * diff;
                chi_squared_numerator += diff;
            }
        }
        chi_squared_numerator as f64 / expected_frequency as f64
    }

    fn get_random_permutation_permutor(max_value: u64, key: u64) -> Vec<u64> {
        let permutor = Permutor::new_with_u64_key(max_value, key);
        permutor.collect()
    }

    fn get_random_permutation_true_random(max_value: u64, rng: &mut impl Rng) -> Vec<u64> {
        let mut values: Vec<u64> = (0..max_value).collect();
        values.shuffle(rng);
        values
    }
}
