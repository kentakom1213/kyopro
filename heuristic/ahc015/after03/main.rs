// 参考: fujikawahiroakiさん
// https://atcoder.jp/contests/ahc015/submissions/36099311

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// input macro
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
    let mut flavors = [0; 100];
    {   
        // `flavor`をセットする
        let f = get!(usize;;);
        for i in 0..100 {
            flavors[i] = f[i];
        }
    }

    // 最も数が多い種類を調べる
    let target_kind = {
        let cnt1 = flavors.iter().filter(|&v| *v == 1).fold(0, |acc, v| acc+1);
        let cnt2 = flavors.iter().filter(|&v| *v == 2).fold(0, |acc, v| acc+1);
        let cnt3 = flavors.iter().filter(|&v| *v == 3).fold(0, |acc, v| acc+1);
        let target_cnt = cnt1.max(cnt2).max(cnt3);
        if target_cnt == cnt1 {
            1
        } else if target_cnt == cnt2 {
            2
        } else {
            3
        }
    };

    for i in 0..99 {
        let p = get!(usize);
        let (r, c) = (p/10, p%10);

        if flavors[i] != target_kind && flavors[i+1] == target_kind {
            println!("L");
        }
        else if flavors[i] == target_kind && flavors[i+1] != target_kind {
            println!("R");
        }
        else {
            println!("B");
        }
    }

    get!(usize);
    println!("F");
}

