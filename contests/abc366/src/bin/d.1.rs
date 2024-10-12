#![allow(non_snake_case)]

use cp_library_rs::{chmax, data_structure::acc3d::acc3D};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [[[u32; N]; N]; N],
    }

    let sum = acc3D(&A);

    // クエリ
    input! {
        Q: usize
    }

    for _ in 0..Q {
        input! {
            lx: Usize1,
            rx: usize,
            ly: Usize1,
            ry: usize,
            lz: Usize1,
            rz: usize,
        }

        let res = sum(lx, rx, ly, ry, lz, rz);

        println!("{res}");
    }
}
