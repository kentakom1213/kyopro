//               B - Savings               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc206/tasks/abc206_b
// ----------------------------------------

// 二分探索

// use proconio::input;

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed");
    let n: usize = n.trim().parse().unwrap();

    let (mut l, mut r) = (0, 1001001001);
    while (r - l) > 1 {
        let mid = (l + r) / 2;
        if mid * (mid+1) / 2 < n {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", l+1);
}
