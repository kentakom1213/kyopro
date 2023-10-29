/*

S: 00100
T: 10011

U: 00001

- 辞書順最小は貪欲法

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

// solve
fn main() {
    let n = get!(usize);
    let s = get!(String);
    let t = get!(String);

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // 貪欲に操作
    let mut ans: Vec<char> = vec![];
    let (mut ham_s, mut ham_t) = (0, 0);
    for (s_i, t_i) in s.chars().zip(t.chars()) {
        ans.push('0');
        if s_i == '1' {
            ham_s += 1;
        }
        if t_i == '1' {
            ham_t += 1;
        }
    }

    // 後ろから、1に変えられるところを探す
    for i in (0..n).rev() {
        if ham_s == ham_t {
            break;
        }
        if ham_s > ham_t {
            if s_chars[i] == '1' && t_chars[i] == '0' {
                ans[i] = '1';
                ham_s -= 1;
                ham_t += 1;
            }
        }
        if ham_s < ham_t {
            if s_chars[i] == '0' && t_chars[i] == '1' {
                ans[i] = '1';
                ham_s += 1;
                ham_t -= 1;
            }
        }
    }

    if ham_s == ham_t {
        for &c in &ans {
            print!("{}", c);
        }
        println!();
    } else {
        println!("-1");
    }
}