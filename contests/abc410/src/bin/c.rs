#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{debug, get};

fn main() {
    let (N, Q) = get!(usize, usize);

    let mut top = 0;
    let mut A: Vec<usize> = (1..=N).collect();

    let nth = |top: usize, i: usize| -> usize { (top + i) % N };

    for _ in 0..Q {
        let query = get!(usize;;);

        match &query[..] {
            &[1, mut p, x] => {
                p -= 1;
                *A.get_mut(nth(top, p)).unwrap() = x;
            }
            &[2, mut p] => {
                p -= 1;
                println!("{}", A.get(nth(top, p)).unwrap());
            }
            &[3, k] => {
                top = (top + k) % N;
            }
            _ => unreachable!(),
        }
    }
}
