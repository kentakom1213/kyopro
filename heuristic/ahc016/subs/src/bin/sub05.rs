// sub05

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// input macro
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    // ($($t:ty);*) => {
    //     (
    //         $(get!($t),)*
    //     )
    // };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

// solve
fn main() {
    let (M, eps) = get!(usize, f64);
    
    println!("25");
    for k in 0..M {
        let k3 = k * 3;
        println!("{}{}", "1".repeat(k3), "0".repeat(300-k3));
    }

    for q in 0..100 {
        let H = get!(String);
        let t = (M-1).min(
            H.chars().filter(|c| *c == '1').count()
        );
        let mut ans = (t as f64) / 3.0 + 0.5;
        ans /= 1.0;
        println!("{}", ans as usize);
    }
}
