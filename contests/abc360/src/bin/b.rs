#![allow(non_snake_case)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars,
        T: Chars
    }

    for w in 1..S.len() {
        let mut ws = vec![vec![]; w];
        for i in 0..S.len() {
            ws[i % w].push(S[i]);
        }
        for i in 0..w {
            if ws[i] == T {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

mod macro_cfor {
    #![allow(dead_code)]
    //! Cスタイルのfor文
    /// cfor! {}
    #[macro_export]
    macro_rules! cfor {
        ($def:stmt ; $fin:expr ; $incr:stmt ;; $bl:block) => {{
            $def
            while $fin {
                $bl
                $incr
            }
        }}
    }
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
