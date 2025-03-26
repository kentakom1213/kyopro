#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::{fastout, input};
use rustc_hash::FxHashSet;

#[fastout]
fn main() {
    input! {
        _N: usize,
        mut pos: (isize, isize),
        S: String
    }

    let mut smoke = FxHashSet::default();
    let mut fire = (0, 0);

    for c in S.chars() {
        smoke.insert(fire);

        match c {
            'N' => {
                fire.0 -= 1;
                pos.0 += 1;
            }
            'W' => {
                fire.1 += 1;
                pos.1 += 1;
            }
            'S' => {
                fire.0 += 1;
                pos.0 -= 1;
            }
            'E' => {
                fire.1 -= 1;
                pos.1 -= 1;
            }
            _ => (),
        }
        debug!(pos, smoke);

        print!("{}", smoke.contains(&pos) as u8);
    }

    println!()
}
