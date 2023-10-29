//               C - Max Even              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc272/tasks/abc272_c
// ----------------------------------------

// [Rustで競技プログラミングの入力をスッキリ記述するマクロ](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8)
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}


// solve
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let (mut odd, mut even) = (vec![], vec![]);
    for &v in &a {
        if v % 2 == 0 {
            even.push(v);
        } else {
            odd.push(v);
        }
    }

    odd.sort(); odd.reverse();
    even.sort(); even.reverse();

    let mut ans: isize = -1;
    if odd.len() >= 2 {
        ans = (odd[0] + odd[1]) as isize;
    }

    if even.len() >= 2 {
        ans = ans.max((even[0] + even[1]) as isize);
    }

    println!("{}", ans);
}
