#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::consts::IINF};
use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        M: usize,
        K: isize,
        A: [isize; N]
    }

    // 候補者をソート
    let A_ = A.iter().cloned().zip(0..).sorted().collect_vec();

    // 候補の位置
    let idx = A_
        .iter()
        .enumerate()
        .fold(FxHashMap::default(), |mut map, (i, &(_, x))| {
            map.insert(x, i);
            map
        });

    debug!(A_);
    debug!(idx);

    // 累積和
    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + A_[i].0;
    }

    // 残りの票数
    let R = K - S[N];

    debug!(R, S);

    // 候補 i が追加でx票とったとき，当選が確定するか
    let is_ok = |i: usize, x: isize| -> bool {
        // 候補iの合計得票
        let si = A[i] + x;
        // 他の候補者に割り当てられる票
        let R_rem = R - x;

        // 自分のインデックス
        let iidx = idx[&i];

        // 得票がsiよりも多いような人数（当選が確定）
        let over = A_.partition_point(|&(a, _)| a <= si);
        let over_cnt = N - over;
        debug!(i, A[i], x, si, over, over_cnt);

        if over_cnt >= M {
            return false;
        }

        // 当選できる残りの人数
        let M_rem = M - over_cnt;
        debug!(R_rem, M_rem);

        // 残りの候補者の開始
        let mut lower = over - M_rem;

        if lower <= iidx {
            // 自分を含んでいたとき
            lower -= 1;
        }

        // 残りの候補者の得票の総和
        let mut rem_total = S[over] - S[lower];

        if lower <= iidx {
            rem_total -= A_[iidx].0;
        }

        debug!(lower, S[over], S[lower], rem_total);
        let total_cnt = (si + 1) * M_rem as isize;
        debug!(total_cnt, rem_total, R_rem);

        // 残りの候補者全員が si + 1 票以上を獲得できるか
        total_cnt - rem_total > R_rem
    };

    // 各候補について考える
    let mut ans: Vec<isize> = vec![];

    for i in 0..N {
        debug!(i);

        if N == M {
            ans.push(0);
            continue;
        }

        let mut ng = -1;
        let mut ok = R + 1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if is_ok(i, mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let res = if ok > R { -1 } else { ok };
        ans.push(res);
    }

    println!("{}", ans.iter().join(" "));
}
