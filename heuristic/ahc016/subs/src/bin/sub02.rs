// sub016

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

// constants
static SEGMENT: usize       = 7;
static ERR_CODE_SIZE: usize = 13;
static V_SIZE: usize        = SEGMENT * ERR_CODE_SIZE;
static CODES: [&str; 2]     = ["0000000000000",
                               "1111111111111"];

// solve
fn main() {
    let (M, eps) = get!(usize, f64);

    println!("{}", V_SIZE);
    for k in 0..M {
        let mut g = String::new();
        for i in 0..7 {
            let code01 = CODES[k >> i & 1];
            g.push_str(code01);
        }
        println!("{}", g);
    }

    for q in 0..100 {
        let H: Vec<char> = get!(String).chars().collect();
        
        // ERR_CODE_SIZEずつ区切って0,1の数をカウントする
        // 多数決で決める
        let mut h = 0;
        for i in 0..SEGMENT {
            let mut cnt_1 = 0;
            for j in 0..ERR_CODE_SIZE {
                let idx = i * ERR_CODE_SIZE + j;
                if H[idx] == '1' {
                    cnt_1 += 1;
                }
            }
            // 多数決
            if cnt_1 > ERR_CODE_SIZE / 2 {
                h += 1 << i;
            }
        }
        let res = (M-1).min(h);
        println!("{}", res);
    }
}
