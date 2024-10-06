use cp_library_rs::debug;
use proconio::input;

#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    // 累積XORを取る
    let mut S = vec![0; N + 1];

    for i in 0..N {
        S[i + 1] = S[i] ^ A[i];
    }

    debug!(S);

    // bitごとに考える
    let mut ans = 0;

    for bit in 0..30 {
        let (cnt0, cnt1): (usize, usize) = S.iter().fold((0, 0), |mut cnt, &a| {
            if a >> bit & 1 == 0 {
                cnt.0 += 1;
            } else {
                cnt.1 += 1;
            }
            cnt
        });

        // 隣接項のXOR
        let adj = A.iter().map(|&a| a >> bit & 1).sum::<usize>() << bit;

        debug!(bit, cnt0, cnt1, adj);

        let tmp = (cnt0 * cnt1) << bit;
        ans += tmp - adj;
    }

    println!("{ans}");
}
