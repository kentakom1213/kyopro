#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const UINF: usize = 1001001001001001001;
const IINF: isize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(Usize1, Usize1, isize); M]
    }

    // グループに分解
    let G = ABC.iter().fold(vec![vec![]; N], |mut g, &(u, v, w)| {
        g[u].push((v, w));
        g[v].push((u, -w));
        g
    });

    // 所属するグループ，グループ内での相対順位
    let (group_id, relative) = {
        let mut g = vec![UINF; N];
        let mut relative = vec![0; N];

        for i in 0..N {
            if g[i] != UINF {
                continue;
            }
            let mut stack = vec![i];
            relative[i] = 0;

            while let Some(u) = stack.pop() {
                for &(v, w) in &G[u] {
                    if g[v] != UINF {
                        continue;
                    }
                    g[v] = i;
                    relative[v] = relative[u] + w;
                    stack.push(v);
                }
            }
        }

        (g, relative)
    };

    debug!(group_id, relative);

    // グループに分割
    let (groups, relative) = {
        let mut groups = vec![vec![]; N];
        let mut mins = vec![IINF; N];

        for (i, &gid) in group_id.iter().enumerate() {
            if gid == UINF {
                groups[i].push(i);
                mins[i] = 0;
            } else {
                groups[gid].push(i);
                mins[gid] = mins[gid].min(relative[i]);
            }
        }

        debug!(mins);

        let relative = (0..N)
            .map(|i| {
                if group_id[i] == UINF {
                    0
                } else {
                    relative[i] - mins[group_id[i]]
                }
            })
            .collect_vec();

        (groups, relative)
    };

    debug!(groups, relative);

    // 埋めるパターンの形状
    let (shapes, mx) = {
        let mut s = vec![0; N];
        let mut mx = vec![0; N];

        for g in 0..N {
            for &i in &groups[g] {
                s[g] |= 1 << relative[i];
                mx[g] = mx[g].max(relative[i] as usize);
            }
        }

        (s, mx)
    };

    debug!(mx);

    if cfg!(debug_assertions) {
        for i in 0..N {
            eprintln!("{:>2}: {:0>16b}", i, shapes[i]);
        }
    }

    // popcountの逆引き
    let pcnt = (0_usize..1 << N).fold(vec![vec![]; N + 1], |mut v, i| {
        v[i.count_ones() as usize].push(i);
        v
    });

    // 答え
    let mut ans = vec![IINF; N];

    let solve = |i: usize, ans: &mut [isize]| {
        if ans[i] != IINF {
            return;
        }

        // dp[x][S] := （頂点iが含まれる連結成分を除き）
        //     x番目までの連結成分の頂点に対応する順位の集合がSになる可能性があるか
        let mut dp = vec![vec![false; 1 << N]; N + 1];

        dp[0][0] = true;

        // 現在埋まっているマスの数
        let mut cnt = 0;

        for j in 0..N {
            if i == j {
                // 埋めない場合
                for &k in &pcnt[cnt] {
                    if dp[j][k] {
                        dp[j + 1][k] = true;
                    }
                }
            } else {
                // 埋める場合
                for &k in &pcnt[cnt] {
                    if dp[j][k] {
                        for f in 0..N - mx[j] {
                            dp[j + 1][k | (shapes[j] << f)] = true;
                        }
                    }
                }
                cnt += groups[j].len();
            }
        }

        let mut ok = vec![];

        for f in 0..N - mx[i] {
            if dp[N][(1 << N) - 1 - (shapes[i] << f)] {
                ok.push(f);
            }
        }

        assert!(ok.len() > 0);

        for &p in &groups[i] {
            if ok.len() == 1 {
                ans[p] = relative[p] + ok[0] as isize;
            }
        }
    };

    for i in 0..N {
        solve(i, &mut ans);
    }

    for i in 0..N {
        if ans[i] == IINF {
            print!("-1 ");
        } else {
            print!("{} ", N - ans[i] as usize);
        }
    }
    println!();
}
