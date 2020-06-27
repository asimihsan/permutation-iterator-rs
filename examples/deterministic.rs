use permutation_iterator::Permutor;

fn main() -> anyhow::Result<()> {
    let max = 10;
    let key: [u8; 32] = [0xBA; 32];
    let permutor = Permutor::new_with_slice_key(max, key)?;
    for permuted in permutor {
        println!("{}", permuted);
    }
    Ok(())
}
