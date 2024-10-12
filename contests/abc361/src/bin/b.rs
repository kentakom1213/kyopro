#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        Amin: [isize; 3],
        Amax: [isize; 3],
        Bmin: [isize; 3],
        Bmax: [isize; 3],
    }

    let has_intersection = |x1min, x1max, x2min, x2max| x1min < x2max && x2min < x1max;

    let x = has_intersection(Amin[0], Amax[0], Bmin[0], Bmax[0]);
    let y = has_intersection(Amin[1], Amax[1], Bmin[1], Bmax[1]);
    let z = has_intersection(Amin[2], Amax[2], Bmin[2], Bmax[2]);

    println!("{}", (x && y && z).yesno());
}
