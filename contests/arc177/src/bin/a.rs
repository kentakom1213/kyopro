#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        mut C: [usize; 6],
        N: usize,
        mut X: [usize; N]
    }

    for i in (0..6).rev() {
        let mut j = 0;
        let mut ng = 0;
        loop {
            // コインがなくなったとき
            if C[i] == 0 {
                break;
            }

            // 1周したとき
            if ng > N {
                break;
            }

            if X[j] < V[i] {
                ng += 1;
            } else {
                ng = 0;
                X[j] -= V[i];
                C[i] -= 1;
            }

            j = (j + 1) % N;
        }
    }

    let isok = X.iter().all(|x| x == &0);

    println!("{}", isok.yesno());
}

const V: [usize; 6] = [1, 5, 10, 50, 100, 500];
