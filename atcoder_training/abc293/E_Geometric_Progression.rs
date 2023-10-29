//        E - Geometric Progression        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc293/tasks/abc293_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

/* 行列累乗 */
const DIM: usize = 2;
type Vec = [usize; DIM];
type Matrix = [[usize; DIM]; DIM];

trait MatrixExp {
    fn e() -> Self;
    fn dot(&self, other: Self, m: usize) -> Self;
    fn pow(&self, e: usize, m: usize) -> Self; 
    fn apply(&self, vec: Vec, m: usize) -> Vec;
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
    fn apply(&self, vec: Vec, m: usize) -> Vec {
        let mut res = [0; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                res[i] += self[i][j] * vec[j] % m;
                res[i] %= m;
            }
        }
        res
    }

    /// ## pow
    /// 行列の累乗を返す（繰り返し2乗法）
    fn pow(&self, mut e: usize, m: usize) -> Self {
        let mut res = Self::e();
        let mut tmp = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                res = tmp.dot(res, m);
            }
            tmp = tmp.dot(tmp, m);
            e >>= 1;
        }
        res
    }

    /// ## dot
    /// 行列のドット積
    fn dot(&self, other: Self, m: usize) -> Self {
        let mut res = [[0; DIM]; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                for k in 0..DIM {
                    res[i][j] += self[i][k] * other[k][j] % m;
                    res[i][j] %= m;
                }
            }
        }
        res
    }
}

// solve
fn main() {
    input! {
        A: usize,
        X: usize,
        M: usize,
    }

    let a = [
        [A, 1],
        [0, 1],
    ];

    let ans = a.pow(X, M).apply([0, 1], M);

    println!("{}", ans[0]);
}