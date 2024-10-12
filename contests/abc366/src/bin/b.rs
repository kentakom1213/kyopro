#![allow(non_snake_case)]

use cp_library_rs::{debug2D, utils::iterutil::IterUtil};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }

    let M = S.iter().map(|s| s.len()).max().unwrap();

    let mut res = vec![vec![' '; N]; M];

    // 転置
    for i in 0..N {
        for j in 0..M {
            if S[i].len() > j {
                res[j][N - i - 1] = S[i][j];
            }
        }
    }

    debug2D!(res);

    // *を埋める
    for i in 0..M {
        let mut filled = false;
        for j in (0..N).rev() {
            if res[i][j] != ' ' {
                filled = true;
            } else if filled {
                res[i][j] = '*';
            }
        }
    }

    for row in res {
        println!("{}", row.iter().join(""));
    }
}
