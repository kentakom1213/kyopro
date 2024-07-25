#![allow(non_snake_case)]

use cp_library_rs::{
    debug,
    geometry::{basic::Line, vec2::Vec2},
    get,
};

fn main() {
    let (x1, y1, x2, y2) = get!(f64, f64, f64, f64);
    let Q = get!(usize);
    let points = get!(f64, f64; Q);

    let line = Line(Vec2(x1, y1), Vec2(x2, y2));

    debug!(line);

    for &(x, y) in &points {
        let Vec2(px, py) = line.projection(Vec2(x, y));
        println!("{} {}", px, py);
    }
}
