#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[usize; W]; H]
    }

    // dp[i][j][k] :=
    //     i-2行目までの操作が確定
    //     i-1行目に操作を (j ? しない : する)
    //     i行目に操作を (j ? しない : する)
    //     とき、i-1行目までに孤立した点が存在しないようにするときの操作回数の最小値
    let mut dp = vec![[[INF; 2]; 2]; H];

    dp[0][0] = [0, 1];

    for i in 1..H {
        // i-2を反転させるか
        for j in 0..2 {
            // i-1を反転させるか
            for k in 0..2 {
                // iを反転させるか
                for l in 0..2 {
                    let isok = (0..W).all(|c| {
                        // 左
                        (c > 0 && A[i - 1][c] == A[i - 1][c - 1])
                            // 右
                            || (c + 1 < W && A[i - 1][c] == A[i - 1][c + 1])
                            // 上
                            || (i > 1 && k ^ A[i - 1][c] == j ^ A[i - 2][c])
                            // 下
                            || k ^ A[i - 1][c] == l ^ A[i][c]
                    })
                    // 最終行のみの判定
                    && (i < H - 1 || (0..W).all(|c| {
                        // 左
                        (c > 0 && A[i][c - 1] == A[i][c])
                            // 右
                            || (c + 1 < W && A[i][c] == A[i][c + 1])
                            // 上
                            || (k ^ A[i - 1][c] == l ^ A[i][c])
                    }));
                    // 孤立していない場合
                    if isok {
                        chmin! {
                            dp[i][k][l],
                            dp[i - 1][j][k] + l,
                        };
                    }
                }
            }
        }
    }

    debug2D!(dp);

    let ans = *dp[H - 1].iter().flatten().min().unwrap();

    if ans == INF {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

const INF: usize = 1001001001001001001;

mod macro_chmin {
    //! chminの実装
    /// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmin {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmin! {
                $a,
                ($b).min($c)
                $(,$other)*
            }
        }};
    }
}
