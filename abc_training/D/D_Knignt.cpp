//              D - Knignt
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc145/tasks/abc145_d
// ----------------------------------------

// 問題読み間違えた、気をつけよう

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;
constexpr ll MOD = 1000000007;

int main() {
    ll x, y; cin >> x >> y;

    // 不可能な移動を排除
    if (
            y % 2 != 0      // yは2の倍数
         || y < 2*x         // 下に行けないため不可能
         || x%2 != (y/2)%2  // 格子にはまらない
    ) {
        cout << 0 << endl;
        return 0;
    }

    // あとは格子の最短経路の本数: (a+b)! / (a!b!)
    y /= 2;
    ll a = max(x, y), b = min(x, y);
    ll res = 1;
    for (ll i = 0; i < b; i++) {
        res = (res * (a + i) / (b - i)) % MOD;
    }

    cout << res << endl;
}