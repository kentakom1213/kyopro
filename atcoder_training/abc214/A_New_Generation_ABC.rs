//          A - New Generation ABC         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc214/tasks/abc214_a
// ----------------------------------------

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed");
    let n: usize = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => unreachable!(),
    };

    match n {
        n if n <= 125 => println!("4"),
        n if n <= 211 => println!("6"),
        n if n <= 214 => println!("8"),
        _ => unreachable!(),
    }
}
