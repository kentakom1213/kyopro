// https://atcoder.jp/contests/arc148/tasks/arc148_b

/*

## 方針
文字列中の区間`S[i..j]`を反転させたときの

 */

fn main() {
    let n: usize = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let s: String = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().to_string()
    };

    const INF: isize = 1e18 as isize;
    let mut rev = [[INF; 5050]; 5050];
}
