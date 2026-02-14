#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};

use cp_library_rs::{debug, utils::consts::INF};
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        P: [(usize, usize); N]
    }

    let mut hs: HashMap<usize, HashSet<usize>> = HashMap::default();
    let mut ws: HashMap<usize, HashSet<usize>> = HashMap::default();

    for (i, &(h, w)) in P.iter().enumerate() {
        hs.entry(h).or_default().insert(i);
        ws.entry(w).or_default().insert(i);
    }

    debug!(hs);
    debug!(ws);

    // 現在の上端，左端
    let mut r = 0;
    let mut c = 0;

    let mut ans = vec![(INF, INF); N];

    for _ in 0..N {
        if let Some(idxs) = hs.get_mut(&(H - r)) {
            // 一つ削除
            if let Some(&i) = idxs.iter().next() {
                idxs.remove(&i);

                let (h, w) = P[i];

                // w 側から削除
                ws.get_mut(&w).unwrap().remove(&i);

                // 答えを設定
                ans[i] = (r, c);

                // 更新
                c += w;

                continue;
            }
        }
        if let Some(idxs) = ws.get_mut(&(W - c)) {
            if let Some(&i) = idxs.iter().next() {
                idxs.remove(&i);

                let (h, w) = P[i];

                hs.get_mut(&h).unwrap().remove(&i);

                ans[i] = (r, c);

                r += h;

                continue;
            }
        }

        unreachable!()
    }

    debug!(ans);

    for (r, c) in ans {
        println!("{} {}", r + 1, c + 1);
    }
}
