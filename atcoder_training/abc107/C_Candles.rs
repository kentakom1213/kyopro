//               C - Candles               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc107/tasks/arc101_a
// ----------------------------------------

/*

## 方針
尺取法を用いる（幅は常に同じなので、厳密には尺取法ではない？）

`l <= 0 <= r` であるように区間を取ってくる
求める値は
```
first = min(abs(xs[l]), xs[r])
second = max(abs(xs[l]), xs[r])
move = 2*first + second
```
とするときの`move`の最小値

 */

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
        k: usize,
        xs: [isize; n],
    }

    if n == 1 {
        println!("{}", xs[0].abs());
        return;
    }

    // 尺取法
    let mut ans = 1e18 as isize;
    for i in 0..=n-k {
        let (l, r) = (i, k+i-1);
        let (lx, rx) = (xs[l], xs[r]);
        let (labs, rabs) = (xs[l].abs(), xs[r].abs());

        if lx * rx >= 0 {
            ans = ans.min( labs.max(rabs) );
        }
        else {
            let first = labs.min(rabs);
            let second = labs.max(rabs);
            ans = ans.min( 2*first + second );
        }
    }

    println!("{}", ans);
}
