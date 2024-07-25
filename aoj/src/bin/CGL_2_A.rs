#![allow(non_snake_case)]

use cp_library_rs::debug;

use cp_library_rs::{
    geometry::{basic::Line, vec2::Vec2},
    get,
};

fn main() {
    let Q = get!(usize);

    for _ in 0..Q {
        let (x0, y0, x1, y1, x2, y2, x3, y3) = get!(f64, f64, f64, f64, f64, f64, f64, f64);

        let s1 = Line(Vec2(x0, y0), Vec2(x1, y1));
        let s2 = Line(Vec2(x2, y2), Vec2(x3, y3));

        debug!(s1, s2);
        debug!((s1.1 - s1.0).cross(s2.1 - s2.0));

        if s1.is_parallel(&s2) {
            println!("2");
        } else if s1.is_orthogonal(&s2) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
