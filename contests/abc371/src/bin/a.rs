#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        ab: char,
        ac: char,
        bc: char
    }

    let is_ab = ab == '<';
    let is_ac = ac == '<';
    let is_bc = bc == '<';

    for abc in (1..=3).permutations(3) {
        let ab = abc[0] < abc[1];
        let ac = abc[0] < abc[2];
        let bc = abc[1] < abc[2];

        if ab == is_ab && ac == is_ac && bc == is_bc {
            let ans = abc.iter().zip('A'..).find(|&(&x, _)| x == 2).unwrap().1;
            println!("{ans}");
        }
    }
}
