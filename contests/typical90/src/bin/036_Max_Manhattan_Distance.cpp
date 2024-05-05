//     036 - Max Manhattan Distance（★5）    
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_aj
// ----------------------------------------

/*

## 方針
45度回転

## 実装
回転行列 `A`
```
A = [[cos, -sin],
     [sin, cos]]
```

*/

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
constexpr ll MAX = 1001001001001001001;

int main() {
    int N, Q; cin >> N >> Q;
    vector<pll> points(N);
    for (auto &[a, b] : points) {
        cin >> a >> b;
    }

    // 座標の最大値、最小値
    ll min_X=MAX, min_Y=MAX;
    ll max_X=-MAX, max_Y=-MAX;

    // 45度回転
    vector<ll> x45(N), y45(N);
    for (int i = 0; i < N; i++) {
        auto [a, b] = points[i];
        x45[i] = a - b;
        y45[i] = a + b;
        chmin(min_X, x45[i]);
        chmax(max_X, x45[i]);
        chmin(min_Y, y45[i]);
        chmax(max_Y, y45[i]);
    }

    // 距離の最大値を求める
    while (Q--) {
        int q; cin >> q; q--;
        ll a = abs(x45[q] - min_X);
        ll b = abs(x45[q] - max_X);
        ll c = abs(y45[q] - min_Y);
        ll d = abs(y45[q] - max_Y);
        cout << max({a, b, c, d}) << endl;
    }
}
