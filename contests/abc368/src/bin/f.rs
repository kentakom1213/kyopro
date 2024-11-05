#![allow(non_snake_case)]

use cp_library_rs::{debug, number_theory::factorize_query::FactorTable};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    // 素因数分解
    let table = FactorTable::new(101010);

    let factors = A.iter().map(|&x| table.factorize(x)).collect_vec();

    debug!(factors);

    let cnt = factors.iter().map(|x| x.len()).collect_vec();

    debug!(cnt);

    let nim = cnt.iter().fold(0, |a, b| a ^ b);

    if nim == 0 {
        println!("Bruno");
    } else {
        println!("Anna");
    }
}
