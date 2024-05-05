//           B - Coloring Matrix           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc298/tasks/abc298_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        mut A: [[usize; N]; N],
        B: [[usize; N]; N],
    }

    let mut ans = false;
    for _ in 0..4 {
        ans |= is_ok(&A, &B);
        rotate(&mut A);
    }

    if ans {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

fn rotate(A: &mut Vec<Vec<usize>>) {
    let N = A.len();
    let mut B = vec![vec![0; N]; N];
    for i in 0..N {
        for j in 0..N {
            B[i][j] = A[N-1-j][i];
        }
    }
    *A = B;
}

fn is_ok(A: &Vec<Vec<usize>>, B: &Vec<Vec<usize>>) -> bool {
    let N = A.len();
    for i in 0..N {
        for j in 0..N {
            if A[i][j] == 1 && B[i][j] == 0 {
                return false;
            }
        }
    }
    true
}