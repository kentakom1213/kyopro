//             A - Rainy Season            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc175/tasks/abc175_a
// ----------------------------------------

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("failed");
    let s = s.trim();

    if s == "RRR" {
        println!("3");
    } else if s == "SSS" {
        println!("0");
    } else if s == "RRS" || s == "SRR" {
        println!("2");
    } else {
        println!("1");
    }
}
