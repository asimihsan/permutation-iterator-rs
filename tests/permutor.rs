#[cfg(test)]
mod tests {
    use permutation_iterator::Permutor;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_different_key_means_different_result() {
        let max = 10;
        let key1 = 0;
        let key2 = 1;

        let values1 = get_permutation_values(max, key1);
        let values2 = get_permutation_values(max, key2);
        assert_ne!(
            values1, values2,
            "expected different permutations given max {}, first key {}, second key {}",
            max, key1, key2
        );
    }

    #[test]
    fn test_same_key_means_same_result() {
        let max = 10;
        let key = 2;

        let values1 = get_permutation_values(max, key);
        let values2 = get_permutation_values(max, key);
        assert_eq!(
            values1, values2,
            "expected same permutations given max {}, key {} used twice",
            max, key
        );
    }

    #[test]
    fn test_small_battery_returns_correct_permutations() {
        for i in 2..50 {
            for key in 0..10 {
                get_permutation_values(i, key);
            }
        }
    }

    #[test]
    fn test_large_max_returns_correct_permutation() {
        get_permutation_values(25_000, 0);
    }

    fn get_permutation_values(max: u64, key: u64) -> Vec<u64> {
        let permutor = Permutor::new_with_u64_key(max, key);
        let result: Vec<u64> = permutor.collect();
        assert_eq!(
            max as usize,
            result.len(),
            "incorrect size result given max {} key {}",
            max,
            key
        );
        let result_unique: HashSet<u64> = HashSet::from_iter(result.clone());
        assert_eq!(
            max as usize,
            result_unique.len(),
            "values aren't unique i.e. not a permutation given max {} key {}",
            max,
            key
        );

        result
    }
}
