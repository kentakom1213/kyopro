#![allow(non_snake_case)]

use cp_library_rs::in_ex;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [[[i32; N]; N]; N],
    }

    // 累積和
    let mut S = vec![vec![vec![0; N + 1]; N + 1]; N + 1];

    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                S[i + 1][j + 1][k + 1] = A[i][j][k] - in_ex!(S; i+1,i; j+1,j; k+1,k);
            }
        }
    }

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

        let res = in_ex!(S; rx,lx; ry,ly; rz,lz);

        println!("{res}");
    }
}
