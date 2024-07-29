#![allow(non_snake_case)]

use std::collections::BTreeSet;

use cp_library_rs::{chmax, data_structure::union_find::UnionFind, debug};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut LRC: [(Usize1, Usize1, usize); Q]
    }

    if !is_spanning(N, &LRC) {
        println!("-1");
        return;
    }

    // まだ結ばれていないN側の頂点
    let mut set = BTreeSet::from_iter(0..N - 1);

    // 重みが小さい方から貪欲に取る
    LRC.sort_by_key(|&(_, _, c)| c);

    let mut uf = UnionFind::new(N);

    let mut ans = 0;

    for &(l, r, c) in &LRC {
        let rm = set.range(l..r).cloned().collect_vec();
        debug!(rm);

        for u in rm {
            if uf.unite(u, u + 1) {
                ans += c;
                set.remove(&u);
            }
        }
    }

    // Q側の頂点と接続
    ans += LRC.iter().map(|(_, _, c)| c).sum::<usize>();

    println!("{ans}");
}

/// 全域性の判定
fn is_spanning(N: usize, LRC: &[(usize, usize, usize)]) -> bool {
    let mut seen = 0;
    for &(l, r, _) in LRC.iter().sorted() {
        if l <= seen {
            chmax!(seen, r);
        } else {
            return false;
        }
    }
    seen >= N - 1
}
