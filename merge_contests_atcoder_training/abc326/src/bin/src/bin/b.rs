use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in n..1000 {
        let d = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect_vec();

        if d[0] * d[1] == d[2] {
            println!("{}", i);
            return;
        }
    }
}
