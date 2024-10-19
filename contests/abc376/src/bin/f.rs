#![allow(non_snake_case)]

use cp_library_rs::utils::consts::INF;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        HT: [(char, Usize1); Q]
    }

    let mut dpl = vec![INF; N];
    dpl[0] = 0;
    let mut dpr = vec![INF; N];
    dpr[1] = 0;

    for &(h, t) in &HT {
        if h == 'L' {
            // 左手をtに移動させる
            for x in 0..N {
                // 右手を補助的に位置xに移動したときのコスト
                // 左手をtに→右手をxに
            }
        } else {
            // 右手をtに移動させる
        }
    }
}
