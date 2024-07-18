use cp_library_rs::get;

fn main() {
    let x = get!(usize);

    println!("{}", x * x * x);
}
