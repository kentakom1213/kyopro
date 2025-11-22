use cp_library_rs::debug;
use proconio::input;

#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    debug!(A);

    // cnt[a] : 現時点で A[i] = a となる i の個数
    let mut cnt = vec![0; N + 1];
    // sum[a] : 現時点で A[i] = a となる i の総和
    let mut sum = vec![0; N + 1];
    let mut ans = 0;

    for (i, &a) in A.iter().enumerate() {
        let cmb = cnt[a] * (cnt[a] + 1) / 2;
        let tmp = i * cnt[a] - sum[a] - cmb;
        debug!(cnt[a], sum[a], tmp);

        ans += tmp;

        cnt[a] += 1;
        sum[a] += i;
    }

    println!("{ans}");
}
