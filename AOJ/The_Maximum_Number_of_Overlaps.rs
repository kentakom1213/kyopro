//      The Maximum Number of Overlaps     
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_5_B&lang=ja
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

fn main() {
    let N = get!(usize);

    let mut field = vec![vec![0; 1010]; 1010];

    for _ in 0..N {
        let (x1, y1, x2, y2) = get!(usize, usize, usize, usize);
        field[x1][y1] += 1;
        field[x1][y2] -= 1;
        field[x2][y1] -= 1;
        field[x2][y2] += 1;
    }

    // 累積和
    for r in 0..1010 {
        for c in 1..1010 {
            field[r][c] += field[r][c - 1];
        }
    }

    for c in 0..1010 {
        for r in 1..1010 {
            field[r][c] += field[r - 1][c];
        }
    }

    if cfg!(debug_assertions) {
        for row in &field[..10] {
            println!("{:?}", &row[..10]);
        }
    }

    let mut ans = 0;
    for r in 0..1010 {
        for c in 0..1010 {
            ans = ans.max(field[r][c]);
        }
    }

    println!("{}", ans);
}
