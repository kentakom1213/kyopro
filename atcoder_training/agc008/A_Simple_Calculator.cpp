//          A - Simple Calculator          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/agc008/tasks/agc008_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    ll x, y;
    cin >> x  >> y;

    ll ans = (ll)1e18;

    // 直接加算するパターン
    if (x <= y) chmin(ans, y-x);

    // 加算→反転
    if (x <= -y) chmin(ans, -y-x+1);

    // 反転→加算
    if (-x <= y) chmin(ans, y-(-x)+1);

    // 反転→加算→反転
    if (-x <= -y) chmin(ans, -y-(-x)+2);

    cout << ans << "\n";
}
