#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1],
        C: [usize; N]
    }

    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    let mut Sc = vec![0; N];
    let mut Sd = vec![0; N];
    dfs1(INF, 0, &G, &C, &mut Sc, &mut Sd);

    debug!(Sc, Sd);

    let mut dp = vec![0; N];
    dfs2(INF, 0, &G, 0, 0, &Sc, &Sd, &mut dp);

    debug!(dp);

    let ans = dp.iter().min().unwrap();

    println!("{ans}");
}

/// 1回目の木dp
/// - `Sc`: 部分木の重み（`C[x]`）の和
/// - `Sd`: 部分木の重み付き距離（`C[x]*d(v,x)`）の和
fn dfs1(p: usize, u: usize, G: &Vec<Vec<usize>>, C: &[usize], Sc: &mut [usize], Sd: &mut [usize]) {
    Sc[u] += C[u];
    for &v in &G[u] {
        if v == p {
            continue;
        }
        dfs1(u, v, G, C, Sc, Sd);
        Sc[u] += Sc[v];
        Sd[u] += Sc[v] + Sd[v];
    }
}

/// rerooting
/// - `p_sum_c`: vの部分木以外の頂点xについて`C[x]`の総和
/// - `p_sum_d`: vの部分木以外の頂点xについて`C[x]*d(v,x)` の総和
fn dfs2(
    p: usize,
    u: usize,
    G: &Vec<Vec<usize>>,
    p_sum_c: usize,
    p_sum_d: usize,
    Sc: &[usize],
    Sd: &[usize],
    dp: &mut [usize],
) {
    dp[u] = Sd[u] + p_sum_d;

    for &v in &G[u] {
        if v == p {
            continue;
        }
        let nxt_sum_c = p_sum_c + Sc[u] - Sc[v];
        let nxt_sum_d = p_sum_d + Sd[u] + nxt_sum_c - (Sc[v] + Sd[v]);
        dfs2(u, v, G, nxt_sum_c, nxt_sum_d, Sc, Sd, dp);
    }
}

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}
