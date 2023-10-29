//               C - ピアニスト高橋君              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc039/tasks/abc039_c
// ----------------------------------------

/*

## 方針
-  パターンマッチ
- 銭湯10個をマッチさせる
- 手書き

0123456789
WBWBWWBWBWBW => Do
WBWWBWBWBWWB => Re
WWBWBWBWWBWB => Mi
WBWBWBWWBWBW => Fa
WBWBWWBWBWWB => So
WBWWBWBWWBWB => La
WWBWBWWBWBWB => Si

*/

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
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

fn main() {
    let s = get!(String);

    match &s[..12] {
        "WBWBWWBWBWBW" => println!("Do"),
        "WBWWBWBWBWWB" => println!("Re"),
        "WWBWBWBWWBWB" => println!("Mi"),
        "WBWBWBWWBWBW" => println!("Fa"),
        "WBWBWWBWBWWB" => println!("So"),
        "WBWWBWBWWBWB" => println!("La"),
        "WWBWBWWBWBWB" => println!("Si"),
        _ => unreachable!(),
    }
}
