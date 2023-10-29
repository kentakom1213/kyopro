use superslice::Ext;

fn main() {
    proconio::input! {
        N: usize,
        M: usize,
        mut A: [usize; N]
    }

    // ソート
    A.sort();

    // 決め打ち二分探索
    let mut ans = 0;

    for i in 0..N {
        let r = A.lower_bound(&(A[i] + M));
        ans = ans.max(r - i);
    }

    println!("{}", ans);
}
