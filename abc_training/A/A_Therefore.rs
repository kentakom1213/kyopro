//            A - ∴ (Therefore)            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc168/tasks/abc168_a
// ----------------------------------------

// solve
fn main() {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed");

    let num: isize = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => unreachable!(),
    };

    match num % 10 {
        2 | 4 | 5 | 7 | 9 => println!("hon"),
        0 | 1 | 6 | 8 => println!("pon"),
        3 => println!("bon"),
        _ => unreachable!(),
    }
}
