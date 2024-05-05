//            C - K-th Substring           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc097/tasks/arc097_a
// ----------------------------------------

// imports
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// main
fn main() {
    input! {
        s: String,
        K: usize,
    }

    let n = s.len();
    let mut top_K = vec!["|", "|", "|", "|", "|", "|"]; // 番兵

    for i in 0..n {
        for j in i + 1..=n {
            let sub = &s[i..j];
            for k in 0..top_K.len() {
                let t = top_K[k];
                if sub == t {
                    break;
                }
                if sub < t {
                    top_K.insert(k, sub);
                    top_K.pop();
                    break;
                }
            }
        }
    }

    println!("{}", top_K[K - 1]);
}
