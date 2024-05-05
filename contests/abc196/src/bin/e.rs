#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::input;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

fn main() {
    input! {
        N: usize,
        AT: [(isize, usize); N],
        Q: usize,
        X: [isize; Q]
    }

    let mut clamp = Clamp {
        a: 0,
        bot: -INF,
        top: INF,
    };

    for &at in &AT {
        match at {
            (a, 1) => {
                clamp.apply_add(a);
            }
            (a, 2) => {
                clamp.apply_max(a);
            }
            (a, 3) => {
                clamp.apply_min(a);
            }
            _ => unreachable!(),
        }
    }

    for &x in &X {
        println!("{}", clamp.eval(x));
    }
}

pub struct Clamp {
    a: isize,
    bot: isize,
    top: isize,
}

impl Clamp {
    fn eval(&self, x: isize) -> isize {
        (x + self.a).max(self.bot).min(self.top)
    }
    fn apply_add(&mut self, a: isize) {
        self.a += a;
        self.bot += a;
        self.top += a;
    }
    fn apply_min(&mut self, bot: isize) {
        self.bot = self.bot.min(bot);
        self.top = self.top.min(bot);
    }
    fn apply_max(&mut self, top: isize) {
        self.bot = self.bot.max(top);
        self.top = self.top.max(top);
    }
}

const INF: isize = 1001001001001001001;
