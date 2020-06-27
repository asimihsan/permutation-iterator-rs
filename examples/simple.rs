use permutation_iterator::Permutor;

fn main() -> anyhow::Result<()> {
    let max = 10;

    // Since we don't pass in a key we will get a random permutation every time we run this.
    // Try it out!
    let permutor = Permutor::new(max)?;

    for (index, permuted) in permutor.enumerate() {
        println!("{} -> {}", index, permuted);
    }

    Ok(())
}
