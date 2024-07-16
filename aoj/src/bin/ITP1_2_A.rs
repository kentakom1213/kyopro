use cp_library_rs::get;

fn main() {
    let (a, b) = get!(isize, isize);

    if a < b {
        println!("a < b");
    } else if a == b {
        println!("a == b");
    } else {
        println!("a > b");
    }
}
