#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        mut Pa: (i64, i64),
        mut Pb: (i64, i64),
        N: i64,
        Ma: usize,
        Mb: usize,
        mut Sa: [(char, i64); Ma],
        mut Sb: [(char, i64); Mb],
    }

    let mut qa = vec![];
    let mut qb = vec![];

    let mut ia = 0;
    let mut ib = 0;

    let mut rem_a = Sa[0].1;
    let mut rem_b = Sb[0].1;

    while ia < Ma && ib < Mb {
        let len = rem_a.min(rem_b);

        qa.push((Sa[ia].0, len));
        qb.push((Sb[ib].0, len));

        rem_a -= len;
        rem_b -= len;

        if rem_a == 0 {
            ia += 1;
            if ia < Ma {
                rem_a = Sa[ia].1;
            }
        }
        if rem_b == 0 {
            ib += 1;
            if ib < Mb {
                rem_b = Sb[ib].1;
            }
        }
    }

    debug!(qa);
    debug!(qb);
    assert_eq!(qa.len(), qb.len());

    let mut ans = 0;

    for (&a, &b) in qa.iter().zip(&qb) {
        assert_eq!(a.1, b.1);

        let n = a.1;

        ans += collision_count(Pa, a.0, Pb, b.0, n);

        // 移動
        let (ar, ac) = step_vec(a.0);
        let (br, bc) = step_vec(b.0);

        Pa.0 += ar * n;
        Pa.1 += ac * n;
        Pb.0 += br * n;
        Pb.1 += bc * n;

        debug!(Pa, Pb);
    }

    println!("{ans}");
}

fn step_vec(c: char) -> (i64, i64) {
    match c {
        'L' => (0, -1),
        'R' => (0, 1),
        'U' => (-1, 0),
        'D' => (1, 0),
        _ => unreachable!(),
    }
}

fn collision_count(Pa: (i64, i64), ca: char, Pb: (i64, i64), cb: char, n: i64) -> i64 {
    let (ar, ac) = step_vec(ca);
    let (br, bc) = step_vec(cb);

    let dr0 = Pa.0 - Pb.0;
    let dc0 = Pa.1 - Pb.1;

    let dvr = ar - br;
    let dvc = ac - bc;

    if dvr == 0 && dvc == 0 {
        return if dr0 == 0 && dc0 == 0 { n } else { 0 };
    }

    let t = if dvr != 0 {
        if (-dr0) % dvr != 0 {
            return 0;
        }
        (-dr0) / dvr
    } else {
        if (-dc0) % dvc != 0 {
            return 0;
        }
        (-dc0) / dvc
    };

    debug!(dr0, dc0, dvr, dvc, t);

    if 1 <= t && t <= n && dr0 + t * dvr == 0 && dc0 + t * dvc == 0 {
        1
    } else {
        0
    }
}
