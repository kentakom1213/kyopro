#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::examples::Add, data_structure::segment_tree::SegmentTree, debug,
};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut S: Chars,
        XC: [(Usize1, char); Q],
    }

    let mut isok = SegmentTree::<Add<usize>>::new(N);

    for i in 0..N - 2 {
        if &S[i..i + 3] == ['A', 'B', 'C'] {
            *isok.get_mut(i).unwrap() = 1;
        }
    }

    debug!(S);
    debug!(isok);

    // クエリ処理
    for &(x, c) in &XC {
        S[x] = c;
        // 再判定
        for i in x.saturating_sub(2)..=(N - 3).min(x + 2) {
            if S[i..i + 3] == ['A', 'B', 'C'] {
                *isok.get_mut(i).unwrap() = 1;
            } else {
                *isok.get_mut(i).unwrap() = 0;
            }
        }

        debug!(S);
        debug!(isok);

        let res = isok.get_range(..);

        println!("{res}");
    }
}
