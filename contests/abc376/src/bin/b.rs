#![allow(non_snake_case)]

use cp_library_rs::{
    chmin, debug,
    utils::consts::{IINF, INF},
};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        HT: [(char, Usize1); Q]
    }

    let mut l = 0;
    let mut r = 1;

    let mut ans = 0;

    for &(h, t) in &HT {
        let tmp = match h {
            'L' => {
                // 全探索
                // 左回り
                let mut lcnt = 0;
                let mut cur = l;
                while cur != t {
                    cur = (cur + N - 1) % N;
                    lcnt += 1;
                    if cur == r {
                        lcnt = IINF;
                        break;
                    }
                }

                // 右回り
                let mut rcnt = 0;
                let mut cur = l;
                while cur != t {
                    cur = (cur + 1) % N;
                    rcnt += 1;
                    if cur == r {
                        rcnt = IINF;
                        break;
                    }
                }

                l = t;
                debug!(lcnt, rcnt);
                lcnt.min(rcnt)
            }
            'R' => {
                // 全探索
                // 左回り
                let mut lcnt = 0;
                let mut cur = r;
                while cur != t {
                    cur = (cur + N - 1) % N;
                    lcnt += 1;
                    if cur == l {
                        lcnt = IINF;
                        break;
                    }
                }

                // 右回り
                let mut rcnt = 0;
                let mut cur = r;
                while cur != t {
                    cur = (cur + 1) % N;
                    rcnt += 1;
                    if cur == l {
                        rcnt = IINF;
                        break;
                    }
                }

                r = t;
                debug!(lcnt, rcnt);
                lcnt.min(rcnt)
            }
            _ => unreachable!(),
        };

        debug!(tmp);
        ans += tmp;
    }

    println!("{}", ans);
}
