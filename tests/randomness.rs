#[cfg(test)]
mod tests {
    use permutation_iterator::Permutor;
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};
    use std::collections::HashMap;

    /// Given a Permutor instance, for a given maximum value over which we're looking for permutations
    /// [0, max), see if each given returned value is evenly distributed.
    ///
    /// TODO not done yet.
    #[test]
    #[ignore]
    fn test_randomness_chi_squared() {
        let mut rng = rand::thread_rng();
        let max_value = 20;
        let min_key = 0;
        let max_key = 2000;
        let mut cell_counts: HashMap<usize, HashMap<u64, u32>> =
            HashMap::with_capacity(max_value as usize);
        for key in min_key..max_key {
            let values = get_random_permutation_permutor(max_value, key);
            //let values = get_random_permutation_true_random(max_value, &mut rng);

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
        let mut chi_squared: f64 = 0.0;
        for (_cell_index, cell_count) in cell_counts {
            for (_cell_value, count) in cell_count {
                let diff = count - expected_frequency;
                let diff = diff * diff;
                chi_squared += diff as f64 / expected_frequency as f64;
            }
        }
        println!("{:?}", chi_squared);
    }

    fn get_random_permutation_permutor(max_value: u64, key: u64) -> Vec<u64> {
        let permutor = Permutor::new_with_u64_key(max_value, key);
        permutor.collect()
    }

    fn get_random_permutation_true_random(max_value: u64, mut rng: &mut impl Rng) -> Vec<u64> {
        let mut values: Vec<u64> = (0..max_value).collect();
        values.shuffle(rng);
        values
    }
}
