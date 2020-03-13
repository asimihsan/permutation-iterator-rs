#[cfg(test)]
mod tests {
    use permutation_iterator::Permutor;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_different_key_means_different_result() {
        // === given ===
        let max = 10;
        let key1 = 0;
        let key2 = 1;

        // === when ===
        let values1 = get_permutation_values(max, key1);
        let values2 = get_permutation_values(max, key2);

        // === then ===
        assert_ne!(
            values1, values2,
            "expected different permutations given max {}, first key {}, second key {}",
            max, key1, key2
        );
    }

    #[test]
    fn test_same_key_means_same_result() {
        // === given ===
        let max = 10;
        let key = 2;

        // === when ===
        let values1 = get_permutation_values(max, key);
        let values2 = get_permutation_values(max, key);

        // === then ===
        assert_eq!(
            values1, values2,
            "expected same permutations given max {}, key {} used twice",
            max, key
        );
    }

    #[test]
    fn test_small_battery_returns_correct_permutations() {
        for max_value in 2..50 {
            for key in 0..10 {
                println!("testing max_value {} key {}", max_value, key);
                get_permutation_values(max_value, key);
            }
        }
    }

    #[test]
    fn test_large_max_returns_correct_permutation() {
        get_permutation_values(25_000, 0);
    }

    fn get_permutation_values(max_value: u64, key: u64) -> Vec<u64> {
        // === given ===
        let permutor = Permutor::new_with_u64_key(max_value, key);

        // === when ===
        let result: Vec<u64> = permutor.collect();

        // === then ===
        assert_eq!(
            max_value as usize,
            result.len(),
            "incorrect size result given max {} key {}",
            max_value,
            key
        );
        let result_unique: HashSet<u64> = HashSet::from_iter(result.clone());
        assert_eq!(
            max_value as usize,
            result_unique.len(),
            "values aren't unique i.e. not a permutation given max {} key {}",
            max_value,
            key
        );

        result
    }
}
