#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;
use rustc_hash::FxHashMap;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N]
    }

    // (末尾の要素を使う, すべて)
    let comb = |seq: &[usize]| -> (Vec<usize>, Vec<usize>) {
        let mut used = vec![];
        let mut unused = vec![0];
        for &x in seq {
            (used, unused) = (unused, used);
            for v in used.iter_mut() {
                unused.push(*v);
                *v += x;
                *v %= M;
            }
        }
        let mut all = used.clone();
        all.extend(unused);
        all.sort_unstable();
        used.sort_unstable();
        (used, all)
    };

    // 前半の結果
    let (pre_used, pre_all) = comb(&A[..N / 2]);
    // 後半の結果
    A[N / 2..].reverse();
    let (post_used, post_all) = comb(&A[N / 2..]);

    let mut ans = 0;

    for a in pre_all {
        let l = post_all.lower_bound(&((M - a) % M));
        let r = post_all.upper_bound(&((M - a) % M));
        ans += r - l;
    }

    // 境界で連続しているもの
    for a in pre_used {
        let l = post_used.lower_bound(&((M - a) % M));
        let r = post_used.upper_bound(&((M - a) % M));
        ans -= r - l;
    }

    println!("{ans}");
}
