//         D - Sum of difference
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc186/tasks/abc186_d

// AC
// ----------------------------------------

// O(n^2)だと間に合わない

// ソート O(nlog(n))
// 累積和で計算

// 0 1 2 3 4
// ->   |1 - 0| + |2 - 0| + |3 - 0| + |4 - 0|  : sum([1, 2, 3, 4]) - 0 * 4
//    + |2 - 1| + |3 - 1| + |4 - 1|            : sum([2, 3, 4]) - 1 * 3
//    + |3 - 2| + |4 - 2|                      : sum([3, 4]) - 2 * 2
//    + |4 - 3|                                : sum([4]) - 3 * 1

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()

int main() {
    int n; cin >> n;
    vector<ll> A(n), S(n);
    rep(i, n) cin >> A[i];

    // ソート O(nlog(n))
    sort(ALL(A));
    // 累積和 O(n)
    S[0] = A[0];
    rep(i, n-1) {
        S[i+1] = S[i] + A[i+1];
    }

    ll res = 0;
    // 和を取る
    rep(i, n) {
        res += S[n-1] - S[i] - A[i] * (n - i - 1);
        // printf("sum:%lld, A[%d]: %lld, (n-i): %d\n", S[n-1] - S[i], i, A[i], (n-i-1));
    }

    cout << res << endl;

}