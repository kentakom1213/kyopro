#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: String
    }

    let N = S.len();
    let mut ans = 0;

    for l in 0..N {
        for r in (l..=N).step_by(2) {
            if l == r {
                continue;
            }

            let mid = (l + r) / 2;

            let left = &S[l..mid];
            let right = &S[mid..r];

            let x = left.chars().next().unwrap();
            let y = right.chars().next().unwrap();

            if !left.chars().all(|c| c == x) {
                continue;
            }
            if !right.chars().all(|c| c == y) {
                continue;
            }

            if to_digit(x) + 1 != to_digit(y) {
                continue;
            }

            ans += 1;
        }
    }
    
    println!("{ans}");
}

fn to_digit(c: char) -> usize {
    c as usize - '0' as usize
}
