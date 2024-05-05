//       D - Shortest Path Queries 2       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc208/tasks/abc208_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int N, M;
ll dp[2][404][404];

int main() {
    cin >> N >> M;
    rep(i, N) rep(j, N) {
        if (i==j) dp[0][i][j] = 0;
        else dp[0][i][j] = 1LL<<60;
    }

    rep(i, M) {
        ll a, b, c; cin >> a >> b >> c;
        dp[0][a-1][b-1] = c;
    }

    // ワーシャルフロイド法
    ll ans = 0;  // <- ここintにしてるだけで WA なんだけど...
    rep(k, N) {          // 経由
        // 片方をクリア
        rep(i, N) rep(j, N) dp[~k&1][i][j] = 0;

        rep(s, N) {      // 始点
            rep(t, N) {  // 終点
                dp[~k&1][s][t] = min(dp[k&1][s][t], dp[k&1][s][k]+dp[k&1][k][t]);
                if (dp[~k&1][s][t] < 1LL<<59) ans += dp[~k&1][s][t];
            }
        }
    }

    cout << ans << endl;
}
