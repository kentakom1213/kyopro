//            C - Many Formulas            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc045/tasks/arc061_a
// ----------------------------------------

/*

## 方針
- 文字の区切り `|S|-1`箇所 を全探索

 */

fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("failed");
        s.trim().to_string()
    };

    let mut ans: usize = 0;

    let n = s.len() - 1;
    for i in 0..(1 << n) {
        // 区間の列挙
        let mut v = vec![0];
        for j in 0..n {
            if ((i >> j) & 1) == 1 {
                v.push(j+1);
            }
        }
        v.push(s.len());

        // 区間を整数に変換
        for j in 0..(v.len()-1) {
            let (l, r) = (v[j], v[j+1]);
            ans += &s[l..r].parse().unwrap();
        }
    }

    println!("{}", ans);
}