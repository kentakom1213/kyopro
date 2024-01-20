#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        S: [Chars; H]
    }

    let mut ans = INF;

    // 行を見る
    {
        for r in 0..H {
            // 各行について
            let mut q = VecDeque::new();
            let mut tmp = 0;

            for c in 0..W {
                match S[r][c] {
                    '.' => {
                        q.push_back('.');
                        tmp += 1;
                        if q.len() > K {
                            if q.pop_front().unwrap() == '.' {
                                tmp -= 1;
                            }
                        }
                    }
                    'o' => {
                        q.push_back('o');
                        if q.len() > K {
                            if q.pop_front().unwrap() == '.' {
                                tmp -= 1;
                            }
                        }
                    }
                    'x' => {
                        q.clear();
                        tmp = 0;
                    }
                    _ => (),
                };
                debug!(tmp, q);

                if q.len() >= K {
                    chmin! {
                        ans,
                        tmp
                    };
                }
            }
        }
    }

    // 列を見る
    {
        for c in 0..W {
            // 各行について
            let mut q = VecDeque::new();
            let mut tmp = 0;

            for r in 0..H {
                match S[r][c] {
                    '.' => {
                        q.push_back('.');
                        tmp += 1;
                        if q.len() > K {
                            if q.pop_front().unwrap() == '.' {
                                tmp -= 1;
                            }
                        }
                    }
                    'o' => {
                        q.push_back('o');
                        if q.len() > K {
                            if q.pop_front().unwrap() == '.' {
                                tmp -= 1;
                            }
                        }
                    }
                    'x' => {
                        q.clear();
                        tmp = 0;
                    }
                    _ => (),
                };
                debug!(tmp, q);

                if q.len() >= K {
                    chmin! {
                        ans,
                        tmp
                    };
                }
            }
        }
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
