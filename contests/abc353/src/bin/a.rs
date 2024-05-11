#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        H: [usize; N]
    }

    let mut id = -1;

    for i in 1..N {
        if H[0] < H[i] {
            id = i as isize + 1;
            break;
        }
    }

    println!("{id}");
}

const INF: usize = 1001001001001001001;
