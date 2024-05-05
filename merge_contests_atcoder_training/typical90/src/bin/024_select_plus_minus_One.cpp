//        024 - Select +／- One（★2）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_x

// 入力受け取りには気をつけよう

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N, K; cin >> N >> K;
    vector<ll> A(N), B(N);
    for (int i = 0; i < N; i++) cin >> A[i];
    for (int i = 0; i < N; i++) cin >> B[i];

    ll diff = 0;
    for (int i = 0; i < N; i++) diff += abs(A[i] - B[i]);


    if (diff <= K && (K - diff) % 2 == 0) cout << "Yes" << endl;
    else cout << "No" << endl;
}