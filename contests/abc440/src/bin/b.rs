use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n]
    }

    let f: Vec<_> = (1..=n).sorted_by_key(|&i| t[i - 1]).collect();

    println!("{} {} {}", f[0], f[1], f[2])
}
