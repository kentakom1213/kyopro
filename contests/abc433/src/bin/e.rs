#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D, utils::iterutil::IterUtil};
use proconio::input;

fn main() {
    input! {
        T: usize
    }
    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input! {
        N: usize,
        M: usize,
        X: [usize; N],
        Y: [usize; M],
    }

    let mut gr = vec![None; N * M + 1];
    let mut gc = vec![None; N * M + 1];

    for (i, &r) in X.iter().enumerate() {
        if gr[r].is_some() {
            println!("No");
            return;
        }
        gr[r] = Some(i);
    }
    for (j, &c) in Y.iter().enumerate() {
        if gc[c].is_some() {
            println!("No");
            return;
        }
        gc[c] = Some(j);
    }

    let mut ok = vec![vec![]; N * M + 1];
    for (i, &r) in X.iter().enumerate() {
        for (j, &c) in Y.iter().enumerate() {
            ok[r.min(c)].push((i, j));
        }
    }

    debug!(ok);

    // 逆順に埋める
    let mut arr = vec![vec![0; M]; N];
    let mut q = vec![];

    for v in (1..=N * M).rev() {
        match (gr[v], gc[v]) {
            (Some(r), Some(c)) => {
                // 置く場所が (r, c) に確定
                arr[r][c] = v;
                // q に追加
                while let Some(pos) = ok[v].pop() {
                    if pos != (r, c) {
                        q.push(pos);
                    }
                }
            }
            (Some(_), None) | (None, Some(_)) => {
                // ok[v] の中から一つ取り出して使う
                if let Some((r, c)) = ok[v].pop() {
                    arr[r][c] = v;
                    // q に追加
                    while let Some(pos) = ok[v].pop() {
                        if pos != (r, c) {
                            q.push(pos);
                        }
                    }
                } else {
                    println!("No");
                    return;
                }
            }
            _ => {
                // q から取り出して置く
                if let Some((r, c)) = q.pop() {
                    arr[r][c] = v;
                    // q に追加
                    while let Some(pos) = ok[v].pop() {
                        if pos != (r, c) {
                            q.push(pos);
                        }
                    }
                } else {
                    println!("No");
                    return;
                }
            }
        }
    }

    debug!(ok);

    println!("Yes");
    for r in 0..N {
        println!("{}", arr[r].iter().join(" "));
    }
}
