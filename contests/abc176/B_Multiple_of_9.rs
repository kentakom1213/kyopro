//            B - Multiple of 9            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc176/tasks/abc176_b
// ----------------------------------------

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed");
    let n = n.trim();

    let sum = n.chars().map(|c| c.to_digit(10).unwrap()).fold(0, |acc, x| (acc+x)%9);
    if sum == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
