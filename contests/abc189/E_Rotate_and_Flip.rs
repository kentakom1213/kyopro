//           E - Rotate and Flip           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc189/tasks/abc189_e
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// # AffineMatrix
/// アフィン変換（3x3行列）
#[derive(Debug, Clone, Copy)]
struct AffineMatrix {
    arr: [[isize; 3]; 3],
}

impl AffineMatrix {
    fn e() -> Self {
        Self {
            arr: [[1, 0, 0],
                  [0, 1, 0],
                  [0, 0, 1]]
        }
    }

    fn rotate90() -> Self {
        Self {
            arr: [[0, -1, 0],
                  [1, 0, 0],
                  [0, 0, 1]]
        }
    }

    fn rotate270() -> Self {
        Self {
            arr: [[0, 1, 0],
                  [-1, 0, 0],
                  [0, 0, 1]]
        } 
    }

    fn mirror_x(p: isize) -> Self {
        Self {
            arr: [[-1, 0, 2*p],
                  [0, 1, 0],
                  [0, 0, 1]]
        }  
    }

    fn mirror_y(p: isize) -> Self {
        Self {
            arr: [[1, 0, 0],
                  [0, -1, 2*p],
                  [0, 0, 1]]
        }  
    }

    fn dot(&self, other: &Self) -> Self {
        let mut arr = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    arr[i][j] += self.arr[i][k] * other.arr[k][j];
                }
            }
        }
        Self { arr }
    }

    fn apply(&self, vec: (isize, isize)) -> (isize, isize) {
        (
            self.arr[0][0] * vec.0 + self.arr[0][1] * vec.1 + self.arr[0][2],
            self.arr[1][0] * vec.0 + self.arr[1][1] * vec.1 + self.arr[1][2],
        )
    }
}

// solve
fn main() {
    // テスト
    // let am1 = AffineMatrix { arr: [[1, 2, 3], [4, 5, 6], [7, 8, 9]] };
    // let am2 = AffineMatrix { arr: [[9, 8, 7], [6, 5, 4], [3, 2, 1]] };
    // {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
    // {{9, 8, 7}, {6, 5, 4}, {3, 2, 1}}
    // eprintln!("{:?}", am1.dot(&am2));
    // eprintln!("{:?}", am2.dot(&am1));

    input! {
        N: usize,
        XY: [(isize, isize); N],
        M: usize,
    }
    let OP = {
        let mut op = vec![];
        for _ in 0..M {
            input!{t: usize}
            if t == 1 {
                op.push(AffineMatrix::rotate270());
            }
            else if t == 2 {
                op.push(AffineMatrix::rotate90());
            }
            else if t == 3 {
                input!{p: isize}
                op.push(AffineMatrix::mirror_x(p));
            }
            else if t == 4 {
                input!{p: isize}
                op.push(AffineMatrix::mirror_y(p));
            }
        }
        op
    };

    // opの累積をとる
    let mut Sop = vec![AffineMatrix::e()];
    for i in 0..M {
        Sop.push(OP[i].dot(&Sop[i]));
    }

    // クエリ処理
    input!{Q: usize}
    for _ in 0..Q {
        input!{
            a: usize,
            b: Usize1,
        }
        let res = Sop[a].apply(XY[b]);
        println!("{} {}", res.0, res.1);
    }
}
