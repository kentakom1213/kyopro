#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        T: usize,
        P: usize,
        mut L: [usize; N]
    }

    L.sort();
    L.reverse();

    // 後ろからP番目の人
    let target = L[P - 1];
    debug!(target);

    let res = T.saturating_sub(target);

    println!("{res}");
}
