//             A - Max Mod Min             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc147/tasks/arc147_a
// ----------------------------------------

use std::collections::BTreeMap;

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
    let n = get!(usize);
    let a = get!(usize;;);

    // 平衡2分木
    let mut mp = BTreeMap::new();

    // aを格納
    for &a_i in &a {
        if mp.contains_key(&a_i) {
            mp.insert(a_i, mp[&a_i]+1);
        } else {
            mp.insert(a_i, 1);
        }
    }

    // カウント
    let mut ans = 0;
    while !mp.is_empty() {
        let (&mini, _) = mp.iter().next().unwrap();
        let (&maxi, &cnt_max) = mp.iter().last().unwrap();

        if mini == maxi {
            break;
        }

        let new = maxi % mini;
        if new == 0 {
            ans += cnt_max;
        } else {
            mp.insert(new, cnt_max);
            ans += cnt_max;
        }
        mp.remove(&maxi);
    }

    println!("{}", ans);
}
