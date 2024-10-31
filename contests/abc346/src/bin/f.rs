#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D, utils::consts::INF};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        S: String,
        T: String
    }

    if {
        let x = S.chars().map(ord).fold(0, |v, x| v | (1 << x));
        let y = T.chars().map(ord).fold(0, |v, x| v | (1 << x));
        // y is not subset of x
        y & x != y
    } {
        println!("0");
        return;
    }

    let ns = S.len();
    let nt = T.len();

    // Sの文字の累積和
    let ss = {
        let mut ss = vec![vec![0; 2 * ns + 1]; 26];
        for (i, c) in S.chars().chain(S.chars()).enumerate() {
            for j in 0..26 {
                ss[j][i + 1] = ss[j][i];
            }
            ss[ord(c)][i + 1] += 1;
        }
        ss
    };

    debug2D!(ss);

    // g(T, k) ⊆ f(S, N) であるか判定する
    let isok = |K: usize| -> bool {
        let mut idx = 0;

        debug!(K);

        for c in T.chars().map(ord) {
            let r = (K - 1) % ss[c][ns] + 1;
            let q = (K - r) / ss[c][ns];
            idx += q * ns;

            let pos = idx % ns;
            let pre = ss[c][pos];
            // k個とる
            let nxt = ss[c][pos..].lower_bound(&(pre + r));
            idx += nxt;

            debug!(c, r, q, pos, pre, nxt, idx);
        }

        idx <= ns * N
    };

    // にぶたん
    let mut ok = 0;
    let mut ng = INF;

    while ng.wrapping_sub(ok) > 1 {
        let mid = ok.wrapping_add(ng) / 2;
        if isok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}

fn ord(c: char) -> usize {
    c as usize - 'a' as usize
}
