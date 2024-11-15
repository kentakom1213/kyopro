#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, char); M]
    }

    let mut is_first = vec![true; N];

    for &(a, b) in &AB {
        println!("{}", (is_first[a] && b == 'M').yesno());
        if b == 'M' {
            is_first[a] = false;
        }
    }
}
