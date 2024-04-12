// 5000 < 2^13

fn main() {
    let N = get!(usize);
    let A = get!(usize;;);

    // 総xorが0にならない場合は拒否
    if A.iter().fold(0, |a, b| a ^ b) != 0 {
        println!("No");
        return;
    }

    // 5001 < N の場合
    // 鳩の巣原理より，同じ数字が2つ以上存在する → これをグループにすれば良い
    if N > 5001 {
        println!("Yes");
        return;
    }

    // dp[i][j] := i番目までの部分列の総xorがjになるものの長さの最小値
    let mut dp = vec![vec![INF; SIZE]; N];

    // 初期化
    dp[0][A[0]] = 1;

    for i in 1..N {
        for j in 0..SIZE {
            chmin! {
                dp[i][j],
                dp[i - 1][j],
                dp[i - 1][j ^ A[i]] + 1,
            };
        }
    }

    // debug2D!(dp);

    if dp[N - 1][0] < N {
        println!("Yes");
    } else {
        println!("No");
    }
}

const INF: usize = 1001001001001001001;
const SIZE: usize = 1 << 13;

mod get_macro {
    //! 入力用マクロ
    //! - 参考：[Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
    /// 入力用マクロ
    #[macro_export]
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
                get_!($t)
            ).collect::<Vec<_>>()
        };
        ($($t:ty),* ; $n:expr) => {
            (0..$n).map(|_|
                get_!($($t),*)
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
                get_!($t ;;)
            ).collect::<Vec<_>>()
        };
    }
}

mod debug_macro {
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}

mod chmin {
    //! chminの実装
    /// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmin {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmin! {
                $a,
                ($b).min($c)
                $(,$other)*
            }
        }};
    }
}
