//            C - Multiple Gift            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc083/tasks/arc088_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    ll X, Y; cin >> X >> Y;
    int ans = 0;
    while (X <= Y) {
        X <<= 1;
        ans++;
    }

    cout << ans << endl;
}
