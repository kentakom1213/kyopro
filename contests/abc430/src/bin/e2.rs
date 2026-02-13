#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        solve()
    }
}

fn solve() {
    input! {
        S: String,
        T: String
    }

    let N = S.len();

    let mut X = T;
    X += &S;
    X += &S;

    let z = z_algorithm(&X.chars().collect::<Vec<_>>());

    debug!(z);

    for i in 0..N {
        if z[N + i] >= N {
            println!("{i}");
            return;
        }
    }

    println!("-1");
}

pub fn z_algorithm(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];

    for i in 0..n {
        let mut j = 0;
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        z[i] = j;
    }

    z
}
