//             D - Choose Me
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc187/tasks/abc187_d

// 解説
// https://atcoder.jp/contests/abc187/editorial/486

// AC (解説)
// ----------------------------------------


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    ll X = 0;
    vector<ll> x(N);
    for (int i = 0; i < N; i++) {
        ll a, b; cin >> a >> b;
        X -= a;  // 選ばなかったとき
        x[i] = a * 2 + b;  // 選んだとき
    }

    // 優先順位の高い順に選んでいく
    sort(x.begin(), x.end());
    ll ans = 0;
    while (X <= 0) {
        X += x.back();
        x.pop_back();
        ans++;
    }

    cout << ans << endl;
}