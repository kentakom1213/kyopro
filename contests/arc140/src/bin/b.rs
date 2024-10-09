#![allow(non_snake_case)]

use cp_library_rs::{data_structure::multiset::MultiSet, debug};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        S: Chars
    }

    // "A*RC*" をカウントする
    let mut q = vec![];
    let mut cnt = vec![-1; N];
    let mut X = MultiSet::new();

    for i in 0..N {
        if S[i] == 'R' {
            cnt[i] = 0;
            q.push((i, i));
        }
    }

    while let Some((l, r)) = q.pop() {
        if l == 0 || r == N - 1 {
            if cnt[l] > 0 {
                X.insert(cnt[l]);
            }
            continue;
        }
        let ll = l - 1;
        let rr = r + 1;
        if S[ll] == 'A' && S[rr] == 'C' && cnt[ll] == -1 && cnt[rr] == -1 {
            cnt[ll] = cnt[l] + 1;
            cnt[rr] = cnt[r] + 1;
            q.push((ll, rr));
        } else {
            if cnt[l] > 0 {
                X.insert(cnt[l]);
            }
        }
    }

    debug!(S);
    debug!(cnt);
    debug!(X);

    // 順に処理
    let mut ans = 0;

    loop {
        // 最大値を取得
        let Some(&max) = X.last() else { break };

        // ARC -> R
        X.remove(&max);
        if max - 1 > 0 {
            X.insert(max - 1);
        }
        ans += 1;

        debug!(X);

        // 最小値を取得
        let Some(&min) = X.first() else {
            break;
        };

        // ARC -> AC
        X.remove(&min);
        ans += 1;

        debug!(X);
    }

    println!("{ans}");
}
