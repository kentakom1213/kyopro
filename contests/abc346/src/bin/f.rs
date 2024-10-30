#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D, utils::consts::NEG1};
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: String,
        T: String
    }

    let ns = S.len();

    // Sの文字の累積和
    let ss = {
        let mut ss = vec![vec![0; ns + 1]; 26];
        for (i, c) in S.chars().enumerate() {
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
        // 何ループ目か
        let mut lop = 0;
        // 何文字目か
        let mut idx = 0;

        debug!(K);

        for c in T.chars() {
            let mut k = K;
            // idx文字目以降に含まれるcの回数
            let after_idx = ss[ord(c)][ns] - ss[ord(c)][idx];
            if after_idx >= k {
                let nxt = ss[ord(c)].partition_point(|&x| x < k);
                idx = nxt + 1;
                continue;
            } else {
                k -= after_idx;
                lop += 1;
                idx = 0;
            }
            debug!(after_idx, k, lop, idx);
            // ループに含まれるtの回数
            let par_loop = ss[ord(c)][ns];

            // 何ループするか
            if k > 0 && par_loop == 0 {
                return false;
            }

            lop += k / par_loop;
            debug!(k, par_loop, k / par_loop);
            // あまりの個数
            k %= par_loop;

            if k == 0 {
                lop -= 1;
                // 最後の位置に
                let nxt = ss[ord(c)].partition_point(|&x| x < par_loop);
                idx = nxt;
                debug!(idx, lop);
                continue;
            }

            debug!(k, lop);
            let nxt = ss[ord(c)].partition_point(|&x| x < k);
            idx = nxt + 1;

            debug!(c, lop, idx);
        }

        lop <= N
    };

    // にぶたん
    let mut ok = NEG1;
    let mut ng = 1001001001001_usize;

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
