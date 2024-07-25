#![allow(non_snake_case)]

use cp_library_rs::{
    geometry::{basic::Segment, vec2::Vec2},
    get,
};

fn main() {
    let Q = get!(usize);

    for _ in 0..Q {
        let (x0, y0, x1, y1, x2, y2, x3, y3) = get!(f64, f64, f64, f64, f64, f64, f64, f64);

        let s1 = Segment(Vec2(x0, y0), Vec2(x1, y1));
        let s2 = Segment(Vec2(x2, y2), Vec2(x3, y3));

        let d = s1.dist_segment(&s2);

        println!("{}", d);
    }
}
