#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
    }

    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    let groups = {
        let mut visited = vec![false; N];
        let mut groups = vec![];
        for u in 0..N {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            groups.push(vec![u]);
            let mut st = vec![u];
            while let Some(u) = st.pop() {
                for &v in &G[u] {
                    if visited[v] {
                        continue;
                    }
                    visited[v] = true;
                    st.push(v);
                    groups.last_mut().unwrap().push(v);
                }
            }
        }
        groups
    };

    debug!(groups);

    // 連結成分ごとに全探索
    let mut ans = 1_usize;

    for cc in &groups {
        let mut C = [0; 20];
        ans *= colorize(0, cc, &mut C, &G);
    }

    println!("{ans}");
}

fn colorize(i: usize, cc: &[usize], C: &mut [usize; 20], G: &Vec<Vec<usize>>) -> usize {
    if i == cc.len() {
        return 1;
    }
    let mut res = 0;
    let u = cc[i];
    for x in 1..=3 {
        // 頂点uを色xで塗る
        C[u] = x;
        if G[u].iter().all(|&v| C[v] == 0 || C[u] != C[v]) {
            res += colorize(i + 1, cc, C, G);
        }
        C[u] = 0;
    }
    res
}

const _INF: usize = 1001001001001001001;

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
