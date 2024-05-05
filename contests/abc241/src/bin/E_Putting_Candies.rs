//           E - Putting Candies           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc241/tasks/abc241_e
// ----------------------------------------

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

// solve
fn main() {
    let (n, k) = get!(usize, usize);
    let a = get!(usize;;);

    // ダブリング配列
    let mut dp = vec![vec![0; n]; 40];

    // 前計算を行う
    for j in 0..n {
        dp[0][j] = a[j];
    }

    for i in 0..39 {
        for j in 0..n {
            // jのj個先の値を求める -> dp[i+1][j]
            dp[i+1][j] = dp[i][j] + dp[i][(j + dp[i][j]) % n];
        }
    }

    // 答えを求める
    let mut ans = 0;
    for i in 0..40 {
        if (k >> i) & 1 == 1 {
            ans += dp[i][ans%n];
        }
    }

    println!("{}", ans);
}
