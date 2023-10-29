use proconio::input;

fn main() {
    input! {
        x: isize,
        y: isize,
    }

    if -3 <= y - x && y - x <= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
