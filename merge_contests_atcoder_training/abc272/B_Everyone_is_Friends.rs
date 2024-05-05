//         B - Everyone is Friends         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc272/tasks/abc272_b
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut X: Vec<Vec<usize>> = Vec::new();
    for i in 0..m {
        input! {
            k: usize,
            arr: [usize; k],
        }
        X.push(arr);
    }

    let mut is_ok = true;
    for i in 1..=n {
        for j in i+1..=n {
            let mut tmp = false;
            for vec in &X {
                tmp |= vec.contains(&i) && vec.contains(&j);
            }
            is_ok &= tmp;
        }
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
