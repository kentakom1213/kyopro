#![allow(non_snake_case)]

use cp_library_rs::chmax;
use proconio::input;

fn main() {
    input! {
        S: String
    }

    let N = S.len();
    let mut ans = 0;

    for l in 0..N {
        for r in l + 1..=N {
            if S[l..r].to_string() == String::from_iter(S[l..r].chars().rev()) {
                chmax! {
                    ans,
                    r - l,
                };
            }
        }
    }

    println!("{ans}");
}
