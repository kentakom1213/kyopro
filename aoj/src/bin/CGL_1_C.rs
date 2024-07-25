#![allow(non_snake_case)]

use cp_library_rs::{
    debug,
    geometry::{basic::Orientation, vec2::Vec2},
    get,
};

fn main() {
    let (ox, oy, ax, ay) = get!(f64, f64, f64, f64);
    let Q = get!(usize);
    let points = get!(f64, f64; Q);

    let O = Vec2(ox, oy);
    let A = Vec2(ax, ay);

    for &(bx, by) in &points {
        let B = Vec2(bx, by);

        let res = match Orientation::calc(O, A, B) {
            Orientation::CounterClockwise => "COUNTER_CLOCKWISE",
            Orientation::Clockwise => "CLOCKWISE",
            Orientation::OnlineBack => "ONLINE_BACK",
            Orientation::OnlineFront => "ONLINE_FRONT",
            Orientation::OnSegment => "ON_SEGMENT",
        };

        println!("{}", res);
    }
}
