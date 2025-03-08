#![allow(non_snake_case)]

use crate::cp_library_rs::{get, utils::iterutil::IterUtil};
use fft::{ntt::FFT::FFT, num::Fp::Fp};

const MOD: u64 = 998244353;

fn main() {
    let (N, M) = get!(usize, usize);
    let mut A = get!(u64;;);
    let mut B = get!(u64;;);

    if N < M {
        A.extend((0..M - N).map(|_| 0));
    } else {
        B.extend((0..N - M).map(|_| 0));
    }

    let fft = FFT(Fp::new(MOD).unwrap());

    let fa = fft.fft(&A).unwrap();
    let fb = fft.fft(&B).unwrap();

    let conv: Vec<_> = fa
        .into_iter()
        .zip(fb.into_iter())
        .map(|(a, b)| a * b % MOD)
        .collect();

    // 復元
    let C = fft.ifft(&conv).unwrap();

    println!(
        "{}",
        C.iter()
            .chain(std::iter::repeat(&0))
            .take(N + M - 1)
            .join(" ")
    );
}

// ==================== fft ====================
pub mod fft {
    #![allow(dead_code)]
    pub mod ntt {
        pub mod FFT {
            //! 高速フーリエ変換の実装
            use crate::fft::num::Fp::Fp;
            /// 高速フーリエ変換の実装
            pub struct FFT(pub Fp);
            impl FFT {
                /// 入力された配列をフーリエ変換する
                pub fn fft(&self, X: &[u64]) -> Result<Vec<u64>, &'static str> {
                    let (i, X) = self.extend_array(X)?;
                    let w = self.0.root_pow2m(i)?;
                    Ok(self.fft_core(X, w))
                }
                /// 入力された配列をフーリエ逆変換する
                pub fn ifft(&self, F: &[u64]) -> Result<Vec<u64>, &'static str> {
                    let (i, F) = self.extend_array(F)?;
                    let w = self.0.root_pow2m(i)?;
                    let winv = self.0.inv(w);
                    let mut res = self.fft_core(F, winv);
                    let n = res.len();
                    // 逆変換後の配列を正規化
                    let inv_n = self.0.inv(n as u64);
                    res.iter_mut().for_each(|v| *v = self.0.mul(*v, inv_n));
                    Ok(res)
                }
                /// フーリエ変換，フーリエ逆変換の共通部分
                ///
                /// - `w`: 回転演算子
                fn fft_core(&self, X: Vec<u64>, w: u64) -> Vec<u64> {
                    let n = X.len();
                    if n == 1 {
                        return X.to_vec();
                    }
                    let (X_even, X_odd): (Vec<u64>, Vec<u64>) = (0..n / 2)
                        .map(|i| {
                            let l = X[i];
                            let r = X[i + n / 2];
                            (
                                self.0.add(l, r),
                                self.0.mul(self.0.sub(l, r), self.0.pow(w, i)),
                            )
                        })
                        .unzip();
                    // 再帰的にFFT
                    let new_w = self.0.pow(w, 2);
                    let Y_even = self.fft_core(X_even, new_w);
                    let Y_odd = self.fft_core(X_odd, new_w);
                    // マージ
                    Y_even
                        .into_iter()
                        .zip(Y_odd.into_iter())
                        .flat_map(|(e, o)| [e, o])
                        .collect()
                }
                /// 長さが 2 べきになるように配列を生成する
                ///
                /// **Arguments**
                /// - `array`: 配列
                ///
                /// **Returns**
                /// - `(i, res)`: 配列の長さを 2^i に拡張した結果
                fn extend_array(&self, array: &[u64]) -> Result<(usize, Vec<u64>), &'static str> {
                    let n = array.len();
                    // 2^i >= n となるような最小の i
                    let mut i = 0;
                    let mut n_ = 1;
                    while n_ < n {
                        i += 1;
                        n_ *= 2;
                    }
                    if i > self.0.k {
                        return Err("The prime p does not have enough factors of 2 in (p - 1).");
                    }
                    // 配列を生成
                    let mut res = array.to_vec();
                    // 残りをゼロ埋め
                    res.extend((0..n_ - n).map(|_| 0));
                    Ok((i, res))
                }
            }
        }
    }
    pub mod num {
        pub mod Fp {
            //! 有限体の実装
            /// 有限体の実装
            #[derive(Debug)]
            pub struct Fp {
                /// mod p
                pub p: u64,
                /// p の原始根
                pub root: u64,
                /// root の逆元
                pub rinv: u64,
                /// p = 2^k * m + 1 となるような m
                pub k: usize,
                /// p = 2^k * m + 1 となるような m
                pub m: u64,
            }
            impl Fp {
                /// 初期化する
                pub fn new(p: u64) -> Result<Self, &'static str> {
                    // p は素数である必要がある
                    if Fp::factorize(p).len() > 1 {
                        return Err("`p` should be prime number.");
                    }
                    // (p - 1) を素因数分解
                    let factors = Fp::factorize(p - 1);
                    // 原始根を探索
                    let root = Self::find_root(p, &factors);
                    // (p-1) に素因数として含まれる 2 の個数
                    let k = factors[0].1 as usize;
                    Ok(Self {
                        p,
                        root,
                        rinv: Self::_inv(p, root),
                        k,
                        m: (p - 1) >> k,
                    })
                }
                /// Fpの原始根を探索する
                fn find_root(p: u64, factors: &Vec<(u64, u64)>) -> u64 {
                    // x が Fp の原始根であるか判定する
                    let is_ok = |x: u64| {
                        factors
                            .iter()
                            .all(|&(pi, _)| Self::_pow(p, x, (p - 1) / pi) != 1)
                    };
                    (1..p)
                        .find(|x| is_ok(*x))
                        // 原始根の存在性より
                        .unwrap()
                }
                /// 素因数分解
                fn factorize(mut x: u64) -> Vec<(u64, u64)> {
                    let mut res = vec![];
                    for p in std::iter::once(2).chain((1..).map(|x| 2 * x + 1)) {
                        if p * p > x {
                            break;
                        }
                        let mut cnt = 0;
                        while x % p == 0 {
                            cnt += 1;
                            x /= p;
                        }
                        if cnt > 0 {
                            res.push((p, cnt));
                        }
                    }
                    if x > 1 {
                        res.push((x, 1));
                    }
                    res
                }
                // ===== 基本的な演算の実装 =====
                /// 0 <= a < p となるように正規化
                fn normalize(p: u64, a: u64) -> u64 {
                    if a < p {
                        return a;
                    }
                    a % p
                }
                /// a + b (mod p)
                fn _add(p: u64, a: u64, b: u64) -> u64 {
                    let a = Self::normalize(p, a);
                    let b = Self::normalize(p, b);
                    let mut res = a + b;
                    if res >= p {
                        res -= p;
                    }
                    res
                }
                /// - a (mod p)
                fn _neg(p: u64, a: u64) -> u64 {
                    let a = Self::normalize(p, a);
                    p - a
                }
                /// a - b (mod p)
                fn _sub(p: u64, a: u64, b: u64) -> u64 {
                    let a = Self::normalize(p, a);
                    let b = Self::normalize(p, b);
                    Self::_add(p, a, Self::_neg(p, b))
                }
                /// a * b (mod p)
                fn _mul(p: u64, a: u64, b: u64) -> u64 {
                    let a = Self::normalize(p, a);
                    let b = Self::normalize(p, b);
                    a * b % p
                }
                /// a ^ b (mod p)
                fn _pow(p: u64, a: u64, mut b: u64) -> u64 {
                    let mut a = Self::normalize(p, a);
                    let mut res = 1;
                    while b > 0 {
                        if b & 1 == 1 {
                            res = Self::_mul(p, res, a);
                        }
                        a = Self::_mul(p, a, a);
                        b >>= 1;
                    }
                    res
                }
                /// a^(-1) mod p
                fn _inv(p: u64, a: u64) -> u64 {
                    Self::_pow(p, a, p - 2)
                }
                // ===== 公開する演算 =====
                /// a + b (mod p)
                pub fn add(&self, a: u64, b: u64) -> u64 {
                    Self::_add(self.p, a, b)
                }
                /// -a (mod p)
                pub fn neg(&self, a: u64) -> u64 {
                    Self::_neg(self.p, a)
                }
                /// a - b (mod p)
                pub fn sub(&self, a: u64, b: u64) -> u64 {
                    Self::_sub(self.p, a, b)
                }
                /// a * b (mod p)
                pub fn mul(&self, a: u64, b: u64) -> u64 {
                    Self::_mul(self.p, a, b)
                }
                /// a ^ b (mod p)
                pub fn pow(&self, a: u64, b: usize) -> u64 {
                    Self::_pow(self.p, a, b as u64)
                }
                /// a^(-1) (mod p)
                pub fn inv(&self, a: u64) -> u64 {
                    Self::_inv(self.p, a)
                }
                /// 2^(1 / 2^a) (mod p)
                pub fn root_pow2m(&self, a: usize) -> Result<u64, &'static str> {
                    if a > self.k {
                        return Err("The prime p does not have enough factors of 2 in (p - 1).");
                    }
                    Ok(Self::_pow(self.p, self.root, self.m << (self.k - a)))
                }
            }
        }
    }
}

// ==================== cp-library-rs ====================
pub mod cp_library_rs {
    #![allow(dead_code)]
    pub mod utils {
        pub mod iterutil {
            //! イテレータのutil
            use std::fmt::{Debug, Display};
            /// イテレータのツール
            pub trait IterUtil: Iterator {
                /// 文字列として結合する
                fn join(&mut self, sep: &str) -> String
                where
                    Self::Item: Display,
                {
                    self.fold(String::new(), |mut s, v| {
                        if s.is_empty() {
                            s = format!("{}", v);
                        } else {
                            s += &format!("{}{}", sep, v);
                        }
                        s
                    })
                }
                /// 文字列として結合する（デバッグ）
                fn join_debug(&mut self, sep: &str) -> String
                where
                    Self::Item: Debug,
                {
                    self.fold(String::new(), |mut s, v| {
                        if s.is_empty() {
                            s = format!("{:?}", v);
                        } else {
                            s += &format!("{}{:?}", sep, v);
                        }
                        s
                    })
                }
            }
            impl<I: Iterator> IterUtil for I {}
        }
    }
    pub mod get {
        //! 入力用マクロ
        //! - 参考：[Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
        /// 入力用マクロ
        #[macro_export]
        macro_rules! get {
            (parse, $val:expr, usize1) => {(get!(parse, $val, usize) - 1)};
            (parse, $val:expr, chars) => {get!(parse, $val, String).chars().collect::<Vec<_>>()};
            (parse, $val:expr, $t:ty) => {$val.parse::<$t>().unwrap()};
            ($p:tt) => {{
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).ok();
                    get!(parse, line.trim(), $p)
            }};
            ($($p:tt),*) => {{
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).ok();
                    let mut iter = line.split_whitespace();
                    ( $(get!(parse, iter.next().unwrap(), $p),)* )
            }};
            ($t:tt ; $n:expr) => {(0..$n).map(|_| get!($t)).collect::<Vec<_>>()};
            ($($t:tt),* ; $n:expr) => {(0..$n).map(|_| get!($($t),*)).collect::<Vec<_>>()};
            ($t:tt ;;) => {{
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).ok();
                    line.split_whitespace().map(|v| get!(parse, v, $t)).collect::<Vec<_>>()
            }};
            ($t:tt ;; $n:expr) => {(0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()};
        }
    }
}
