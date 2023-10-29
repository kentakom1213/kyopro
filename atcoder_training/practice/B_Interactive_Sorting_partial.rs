


// https://atcoder.jp/contests/practice/tasks/practice_2

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
    let (n, q) = get!(usize, usize);

    // ソートする配列
    let ascii = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let seq: Vec<char> = ascii[..n].chars().collect();
    let sorted = merge_sort(&seq);

    println!("! {}", sorted.iter().collect::<String>());
}

/// ## 比較関数
/// `a <= b`である場合に`true`を返す。
fn less_equal(a: char, b: char) -> bool {
    // クエリを投げる
    println!("? {} {}", a, b);

    // 答えを得る
    let res = get!(char);

    res == '<'
}

/// クエリを投げながらマージソート
fn merge_sort(arr: &[char]) -> Vec<char> {
    let len = arr.len();
    if len == 1 {
        arr.to_vec()
    } else {
        // 左右に分割
        let split_left = &arr[..len/2];
        let split_right = &arr[len/2..];

        // 左、右をソート
        let left = merge_sort(split_left);
        let right = merge_sort(split_right);

        // マージ
        let mut merged = vec![];
        let (mut i, mut j) = (0, 0);

        while i < left.len() || j < right.len() {
            if j == right.len() || i < left.len() && less_equal(left[i], right[j]) {
                merged.push(left[i]);
                i += 1;
            } else {
                merged.push(right[j]);
                j += 1;
            }
        }

        merged
    }
}
