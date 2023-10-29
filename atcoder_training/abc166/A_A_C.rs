//                 A - A?C                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc166/tasks/abc166_a
// ----------------------------------------

fn main() {
    proconio::input! {
        s: String,
    }
    match s {
        x if x == "ABC" => println!("ARC"),
        x if x == "ARC" => println!("ABC"),
        _ => unreachable!(),
    }
}
