use cp_library_rs::get;

fn main() {
    let (a, b) = get!(usize, usize);

    println!("{} {}", a * b, 2 * (a + b));
}
