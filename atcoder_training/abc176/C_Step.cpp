//                 C - Step
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc176/tasks/abc176_c

// C++は整数型の大きさに気をつけよう

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    vector<ll> A(N);
    for (auto &a : A) cin >> a;

    // 区間 A[0] ~ A[i]の最大値を求める
    vector<ll> seg_max(N);
    ll max = 0;
    for (ll i = 0; i < N; i++) {
        if (A[i] > max) max = A[i];
        seg_max[i] = max;
    }

    // 差を求める
    ll res = 0;
    for (ll i = 0; i < N; i++) {
        res += seg_max[i] - A[i];
    }

    cout << res << endl;
}