//                A - mod M                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc148/tasks/arc148_a
// ----------------------------------------

/*

## 解法
- 「2以上の整数M」であるから、2を選ぶ
- 2で割ったあまりが全て一致するとき、

 */

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    // 単一の値の受け取り
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    // 複数の値をパターンマッチ
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
    // 複数行の入力を受け取り
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    // 複数の値を複数行で受け取り
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    // 1行をベクタで受け取り
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    // 複数行をベクタで受け取り
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

fn main() {
    let n = get!(usize);
    let a = get!(isize;;);

    let mut g = 0;
    for i in 0..n-1 {
        g = gcd(g, (a[i] - a[i+1]).abs());
    }

    if g == 1 {
        println!("2");
    } else {
        println!("1");
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}
