#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        mut L: usize,
        R: usize,
    }

    L -= 1;

    // gcdがkであるようなx,yの組の個数
    let mut f = vec![0; R + 1];

    let mut ans = 0;

    for k in (2..=R).rev() {
        f[k] = (R / k - L / k).pow(2);
        cfor! {let mut i = 2; k * i <= R; i += 1;; {
            f[k] -= f[i * k];
        }}
        ans += f[k];
    }

    // xがyの倍数になっているようなものを除く

    for i in L + 1..=R {
        if i == 1 {
            continue;
        }
        ans -= R / i * 2 - 1;
    }

    println!("{ans}");
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
