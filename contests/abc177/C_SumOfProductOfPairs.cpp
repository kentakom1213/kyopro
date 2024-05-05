//       C - Sum of product of pairs
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc177/tasks/abc177_c

// AC
// ----------------------------------------

// 全探索 O(n^2)


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
const int MOD = 1000000007;

int main() {
    int N; cin >> N;
    vector<ll> A(N);
    for (int i = 0; i < N; i++) cin >> A[i];

    // 逆向きの累積和
    vector<ll> S(N);
    S[N-1] = A[N-1];
    for (int i = N-2; i >= 0; i--) {
        S[i] = (S[i+1] + A[i]) % MOD;
    }

    ll res = 0;
    for (int i = 0; i < N-1; i++) {
        res = (res + A[i] * S[i+1]) % MOD;
    }

    cout << res << endl;
}