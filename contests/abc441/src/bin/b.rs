#![allow(non_snake_case)]

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        N: usize,
        M: usize,
        S: String,
        T: String,
        Q: usize,
        W: [String; Q]
    }

    let tak: FxHashSet<char> = S.chars().collect();
    let aok: FxHashSet<char> = T.chars().collect();

    for q in W {
        let w: FxHashSet<char> = q.chars().collect();

        let istak = w.is_subset(&tak);
        let isaok = w.is_subset(&aok);

        if istak && isaok {
            println!("Unknown");
        } else if istak {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    }
}
