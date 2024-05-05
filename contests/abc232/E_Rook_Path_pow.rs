//              E - Rook Path
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc232/tasks/abc232_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

// 定数
const MOD: usize = 998244353;

/* 行列累乗 */
const DIM: usize = 4;
type Vec = [usize; DIM];
type Matrix = [[usize; DIM]; DIM];

trait MatrixExp {
    fn e() -> Self;
    fn dot(&self, other: Self) -> Self;
    fn pow(&self, e: usize) -> Self; 
    fn apply(&self, vec: Vec) -> Vec;
}

impl MatrixExp for Matrix {
    /// ## e
    /// 単位行列を返す
    fn e() -> Self {
        let mut res = [[0; DIM]; DIM];
        for i in 0..DIM {
            res[i][i] = 1;
        }
        res
    }

    /// ## apply
    /// ベクトル`x`と行列`A`について、`Ax`を返す
    fn apply(&self, vec: Vec) -> Vec {
        let mut res = [0; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                res[i] += self[i][j] * vec[j] % MOD;
                res[i] %= MOD;
            }
        }
        res
    }

    /// ## pow
    /// 行列の累乗を返す（繰り返し2乗法）
    fn pow(&self, mut e: usize) -> Self {
        let mut res = Self::e();
        let mut tmp = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                res = tmp.dot(res);
            }
            tmp = tmp.dot(tmp);
            e >>= 1;
        }
        res
    }

    /// ## dot
    /// 行列のドット積
    fn dot(&self, other: Self) -> Self {
        let mut res = [[0; DIM]; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                for k in 0..DIM {
                    res[i][j] += self[i][k] * other[k][j] % MOD;
                    res[i][j] %= MOD;
                }
            }
        }
        res
    }
}

/// ## 解説
/// - 状態数を削減してDP
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
    }

    // 初期値を定義
    let mut a = [0, 0, 0, 0];
    match (x1 == x2, y1 == y2) {
        (true, true)   => a[0] = 1,
        (false, true)  => a[1] = 1,
        (true, false)  => a[2] = 1,
        (false, false) => a[3] = 1,
    };

    // 更新を表す行列
    let M = [
        [0, 1, 1, 0],
        [H-1, H-2, 0, 1],
        [W-1, 0, W-2, 1],
        [0, W-1, H-1, H+W-4],
    ];

    // 行列累乗で解を求める
    let res = M.pow(K).apply(a);
    let ans = res[0];

    println!("{}", ans);
}
