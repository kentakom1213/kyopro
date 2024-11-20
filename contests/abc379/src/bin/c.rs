#![allow(non_snake_case)]

use cp_library_rs::debug;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        X: [usize; M],
        A: [usize; M]
    }

    if A.iter().sum::<usize>() != N {
        println!("-1");
        return;
    }

    let mut XA = X
        .iter()
        .cloned()
        .zip(A.iter().cloned())
        .sorted()
        .collect_vec();

    XA.push((N + 1, 0));

    let mut cost = 0;

    for i in 0..M {
        let (l, n) = XA[i];
        let (r, _) = XA[i + 1];

        // 移動コスト
        cost += (0..r - l).sum::<usize>();

        if r - l < n {
            // 余分なら持ち越し
            let over = n - (r - l);
            XA[i].1 -= over;
            XA[i + 1].1 += over;
            cost += over * (r - l);
        } else if r - l > n {
            // 足りないなら失敗
            println!("-1");
            return;
        }

        debug!(cost, XA);
    }

    if XA[M].1 > 0 {
        println!("-1");
        return;
    }

    println!("{cost}");
}
