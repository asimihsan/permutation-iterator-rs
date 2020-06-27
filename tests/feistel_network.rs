#[cfg(test)]
mod tests {
    use permutation_iterator::FeistelNetwork;

    #[test]
    fn test_new_ten_input_needs_four_bits() {
        // === given ==
        let permutor = FeistelNetwork::new(10).expect("expected new FeistelNetwork");

        // === then ===
        assert_eq!(permutor.half_width, 2);
    }

    #[test]
    fn test_new_eight_input_needs_four_bits() {
        // === given ==
        let permutor = FeistelNetwork::new(8).expect("expected new FeistelNetwork");

        // === then ===
        assert_eq!(permutor.half_width, 2);
    }

    #[test]
    fn test_new_ten_input_right_mask() {
        // === given ==
        let permutor = FeistelNetwork::new(10).expect("expected new FeistelNetwork");

        // === then ===
        // Since we have 4 bits of output, the right two bits are 0011, i.e. 3, i.e. 0x3
        assert_eq!(permutor.right_mask, 0x3);
    }

    #[test]
    fn test_new_ten_input_left_mask() {
        // === given ==
        let permutor = FeistelNetwork::new(10).expect("expected new FeistelNetwork");

        // === then ===
        // Since we have 4 bits of output, the first two bits are 1100, i.e. 12, i.e. 0xC
        assert_eq!(permutor.left_mask, 0xC);
    }
}
