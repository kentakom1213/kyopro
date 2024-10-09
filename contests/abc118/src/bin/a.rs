use proconio::input;

#[allow(non_snake_case)]

fn main() {
    input! {
        A: isize,
        B: isize,
    }

    if B % A == 0 {
        println!("{}", A + B);
    } else {
        println!("{}", B - A);
    }
}
