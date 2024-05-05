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
        S: Chars
    }

    let N = S.len();

    // 消せる文字の個数の偶奇
    // S[0] == S[-1] のとき：
    //    - 文字列長が偶数なら偶数個
    //    - 文字列長が奇数なら奇数個
    // S[0] != S[-1] のとき：
    //    - 文字列長が偶数なら奇数個
    //    - 文字列長が奇数なら偶数個

    // 消せる文字列が奇数個のとき → Firstの勝ち
    // 消せる文字列が偶数個のとき → Secondの勝ち

    let is_second = (S[0] == S[N - 1]) ^ (S.len() % 2 == 0);

    if is_second {
        println!("Second");
    } else {
        println!("First");
    }
}

const INF: usize = 1001001001001001001;
