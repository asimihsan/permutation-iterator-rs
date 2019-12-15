use permutation_iterator::RandomPairPermutor;

fn main() {
    let xs = [1, 2, 3];
    let ys = [4, 5, 6, 7, 8];

    let permutor = RandomPairPermutor::new(xs.len() as u32, ys.len() as u32);
    for (i, j) in permutor {
        println!("({}, {})", xs[i as usize], ys[j as usize]);
    }
}
