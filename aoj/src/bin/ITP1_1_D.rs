use cp_library_rs::get;

fn main() {
    let S = get!(usize);

    let s = S % 60;
    let m = (S / 60) % 60;
    let h = S / 3600;

    println!("{}:{}:{}", h, m, s);
}
