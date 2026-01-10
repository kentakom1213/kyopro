use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let mut ans = x;

    for _ in 0..y {
        ans *= 2;
    }

    println!("{ans}");
}
