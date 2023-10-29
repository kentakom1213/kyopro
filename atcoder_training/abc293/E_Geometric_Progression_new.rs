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

#[derive(Debug, Clone)]
struct MatrixExpMod {
    order: usize,  // MODの値
    dim: usize,  // 次元
    arr: Vec<Vec<usize>>,  // 行列
}

impl MatrixExpMod {
    /// ## e
    /// 単位行列を返す
    fn e(order: usize, dim: usize) -> Self {
        let mut arr = vec![vec![0; dim]; dim];
        for i in 0..dim {
            arr[i][i] = 1;
        }
        Self { order, dim, arr }
    }

    pub fn new(order: usize, arr: Vec<Vec<usize>>) -> Self {
        Self { order, dim: arr.len(), arr }
    }

    /// ## apply
    /// ベクトル`x`と行列`A`について、`Ax`を返す
    pub fn apply(&self, vec: &Vec<usize>) -> Vec<usize> {
        let mut res = vec![0; self.dim];
        for i in 0..self.dim {
            for j in 0..self.dim {
                res[i] += self.arr[i][j] * vec[j] % self.order;
                res[i] %= self.order;
            }
        }
        res
    }

    /// ## pow
    /// 行列の累乗を返す（繰り返し2乗法）
    pub fn pow(&self, mut e: usize) -> Self {
        let mut res = Self::e(self.order, self.dim);
        let mut tmp = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                res = tmp.dot(&res);
            }
            tmp = tmp.dot(&tmp);
            e >>= 1;
        }
        res
    }

    /// ## dot
    /// 行列のドット積
    fn dot(&self, other: &Self) -> Self {
        let mut arr = vec![vec![0; self.dim]; self.dim];
        for i in 0..self.dim {
            for j in 0..self.dim {
                for k in 0..self.dim {
                    arr[i][j] += self.arr[i][k] * other.arr[k][j] % self.order;
                    arr[i][j] %= self.order;
                }
            }
        }
        Self { order: self.order, dim: self.dim, arr }
    }
}

// solve
fn main() {
    input! {
        A: usize,
        X: usize,
        M: usize,
    }

    let a = MatrixExpMod::new(
        M,
        vec![
            vec![A, 1],
            vec![0, 1],
        ]
    );

    let ans = a.pow(X).apply(&vec![0, 1]);

    println!("{}", ans[0]);
}