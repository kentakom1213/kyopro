#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input!(N: usize);

    let mut B = vec![];

    for _ in 0..N {
        input! {
            X: usize,
            Y: usize,
            Z: usize,
            M: usize,
            xyz: [(usize, usize, usize); M]
        }

        let mut xmin = X;
        let mut xmax = 0;
        let mut ymin = Y;
        let mut ymax = 0;
        let mut zmin = Z;
        let mut zmax = 0;

        for &(x, y, z) in &xyz {
            chmin!(xmin, x);
            chmax!(xmax, x);
            chmin!(ymin, y);
            chmax!(ymax, y);
            chmin!(zmin, z);
            chmax!(zmax, z);
        }

        B.push(xmin);
        B.push(X - xmax - 1);
        B.push(ymin);
        B.push(Y - ymax - 1);
        B.push(zmin);
        B.push(Z - zmax - 1);
    }

    debug!(B);

    let nim = B.iter().fold(0, |acc, x| acc ^ x);

    debug!(nim);

    if nim > 0 {
        println!("WIN");
    } else {
        println!("LOSE");
    }
}

mod macro_chmin {
    #![allow(dead_code)]
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

mod macro_chmax {
    #![allow(dead_code)]
    //! chmaxの実装
    /// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmax {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmax! {
                $a,
                ($b).max($c)
                $(,$other)*
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
