#![allow(non_snake_case)]

use std::{collections::BTreeMap, usize};

use cp_library_rs::debug;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut map: BTreeMap<(usize, i8), usize> = (1..=N)
        .flat_map(|c| [((c, END), c), ((c, START), c)])
        .collect();

    map.insert((0, START), 0);
    map.insert((N + 1, END), 0);

    debug!(map);

    let mut cnt = vec![1; N + 1];

    for _ in 0..Q {
        input!(t: usize);

        if t == 1 {
            input! {
                x: usize,
                c: usize,
            }

            let prv_l;
            let prv_c;

            let resl = 'l: {
                let (&(l, _), &lc) = map.range(..(x, 0)).next_back().unwrap();
                prv_l = l;
                prv_c = lc;

                // 左側の色を変える
                if lc != c {
                    *map.get_mut(&(l, END)).unwrap() = c;
                }

                // 左側の色が同じ時
                if let Some((&(ll, _), &llc)) = map.range(..(l, END)).next_back() {
                    if llc == c {
                        map.remove(&(l, END));
                        map.remove(&(ll, START));
                        break 'l Some(ll);
                    }
                }

                None
            };

            let prv_r;

            let resr = 'r: {
                let (&(r, _), &rc) = map.range((x, 0)..).next().unwrap();
                prv_r = r;

                // 右側の色を変える
                if rc != c {
                    *map.get_mut(&(r, START)).unwrap() = c;
                }

                // 右側の色が同じ時
                if let Some((&(rr, _), &rrc)) = map.range((r, START + 1)..).next() {
                    if rrc == c {
                        map.remove(&(r, START));
                        map.remove(&(rr, END));
                        break 'r Some(rr);
                    }
                }

                None
            };

            debug!(resl, resr);
            if let Some((ll, rr)) = resl.zip(resr) {
                map.remove(&(ll, START));
                map.remove(&(rr, END));
            }

            debug!(map);

            // 区間の更新
            // 元の色を引く
            cnt[prv_c] -= prv_r - prv_l + 1;

            // 新しい色を足す
            cnt[c] += prv_r - prv_l + 1;

            debug!(cnt);
        } else {
            input! {
                c: usize
            }

            println!("{}", cnt[c]);
        }
    }
}

const START: i8 = 100;
const END: i8 = -100;
