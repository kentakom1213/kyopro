#![allow(non_snake_case)]

use cp_library_rs::{
    chmax, chmin,
    utils::{consts::INF, yesno::YesNo},
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        P: usize,
        CS: [(usize, String); N]
    }

    let mut sale_min = INF;
    let mut sold_max = 0;

    for (x, state) in &CS {
        if state == "on_sale" {
            chmin! {
                sale_min,
                *x,
            };
        } else {
            chmax! {
                sold_max,
                *x,
            };
        }
    }

    println!("{}", (sold_max < P && P < sale_min).yesno());
}