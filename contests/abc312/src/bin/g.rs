#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1]
    }

    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(a, b)| {
        g[a].push(b);
        g[b].push(a);
        g
    });

    let mut sub = vec![0; N];
    let mut ans = 0;
    rec(INF, 0, &mut sub, &G, &mut ans);

    println!("{ans}");
}

fn rec(p: usize, u: usize, sub: &mut [usize], G: &Vec<Vec<usize>>, ans: &mut usize) {
    sub[u] += 1;

    // 葉の場合
    if G[u].len() == 1 && G[u][0] == p {
        return;
    }

    let mut childs = vec![];

    for &v in &G[u] {
        if v == p {
            continue;
        }
        rec(u, v, sub, G, ans);
        childs.push(sub[v]);
        sub[u] += sub[v];
    }

    let p_sum = G.len() - sub[u];
    childs.push(p_sum);

    debug!(u, childs);

    // 積の和
    let m = childs.len();

    // dp[i][j] := i番目までみてj個選んだときの積の和
    let mut dp = vec![[0; 4]; m + 1];
    dp[0][0] = 1;

    for i in 0..m {
        for j in 0..4 {
            dp[i + 1][j] += dp[i][j];
            if j < 3 {
                dp[i + 1][j + 1] = dp[i][j] * childs[i];
            }
        }
    }

    debug2D!(dp);

    *ans += dp[m][3];
}

const INF: usize = 1001001001001001001;

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
