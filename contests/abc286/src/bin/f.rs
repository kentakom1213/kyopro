#![allow(non_snake_case)]

use itertools::Itertools;

use crate::crt::garner_algorithm;

fn main() {
    let M = P.iter().sum::<usize>();
    println!("{M}");

    assert!(M <= 110);
    assert!(P.iter().product::<usize>() > 1000000000);

    // P[i]の大きさの閉路を含むfunctional graphを構築
    let A = {
        let mut a = vec![0; M];
        let mut s = 0;
        for &p in &P {
            a[s] = s + p;
            for i in 0..p - 1 {
                a[s + i] = s + i + 2;
            }
            a[s + p - 1] = s + 1;
            s += p;
        }
        a
    };

    println!("{}", A.iter().join(" "));

    // 結果の受取り
    let B = get!(usize;;);

    // 剰余を求める
    let rems = {
        let mut s = 0;
        let mut r = vec![];
        for &p in &P {
            debug!(B[s], s);
            let m = (B[s] - 1 - s) % p;
            r.push(m);
            s += p;
        }
        r
    };

    debug!(rems);

    // CRT
    let ans = garner_algorithm(&rems, &P);

    println!("{ans}");
}

const P: [usize; 9] = [4, 5, 7, 9, 11, 13, 17, 19, 23];

mod crt {
    /// 拡張ユークリッド互除法
    /// - `ax + by = gcd(a, b)` を満たす `(x, y, gcd(a,b))` を返す
    pub fn ext_gcd(a: isize, b: isize) -> (isize, isize, isize) {
        if b == 0 {
            return (1, 0, a);
        }
        let (q, r) = (a / b, a % b);
        let (xx, yy, d) = ext_gcd(b, r);
        let x = yy;
        let y = xx - q * yy;
        (x, y, d)
    }

    /// 拡張ユークリッド互除法によるモジュラ逆元の計算
    /// - `ax ≡ 1 (mod m)` を満たす`x`を求める．
    /// - `m`が素数である必要はないが，`a`と`m`は互いに素である必要がある．
    pub fn inv(a: isize, m: isize) -> Option<isize> {
        let (x, _, d) = ext_gcd(a, m);
        (d == 1).then_some(x.rem_euclid(m))
    }

    /// 中国剰余定理
    ///
    /// `rems: [r1, r2, ..., rn], mods: [m1, m2, ..., mn]`に対し，
    /// - `x ≡ r1 (mod m1)`
    /// - `x ≡ r2 (mod m2)`
    /// - ...
    /// - `x ≡ rn (mod mn)`
    ///
    /// を満たす`x`を求める．
    /// - ただし，任意の`(i,j)`に対し`mi`と`mj`は互いに素である必要がある．
    pub fn garner_algorithm(rems: &[usize], mods: &[usize]) -> usize {
        let mut m = 1;
        let mut x = (rems[0] % mods[0]) as isize;

        for i in 0..rems.len() {
            let (ri, mi) = (rems[i] as isize, mods[i] as isize);
            let Some(inv_m) = inv(m, mi) else {
                panic!("For all (i,j), gcd(mi, mj) must be 1.")
            };
            let t = ((ri - x) * inv_m).rem_euclid(mi);
            x += t * m;
            m *= mi;
        }
        x as usize
    }
}

mod macro_get {
    #![allow(dead_code)]
    //! 入力用マクロ
    //! - 参考：[Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
    /// 入力用マクロ
    #[macro_export]
    macro_rules! get {
        ($t:ty) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line.trim().parse::<$t>().unwrap()
            }
        };
        ($($t:ty),*) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let mut iter = line.split_whitespace();
                (
                    $(iter.next().unwrap().parse::<$t>().unwrap(),)*
                )
            }
        };
        ($t:ty ; $n:expr) => {
            (0..$n).map(|_|
                get_!($t)
            ).collect::<Vec<_>>()
        };
        ($($t:ty),* ; $n:expr) => {
            (0..$n).map(|_|
                get_!($($t),*)
            ).collect::<Vec<_>>()
        };
        ($t:ty ;;) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line.split_whitespace()
                    .map(|t| t.parse::<$t>().unwrap())
                    .collect::<Vec<_>>()
            }
        };
        ($t:ty ;; $n:expr) => {
            (0..$n).map(|_|
                get_!($t ;;)
            ).collect::<Vec<_>>()
        };
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
