//           C - 1111gal password          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc242/tasks/abc242_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int MAX_N = 1010101;
ll dp[MAX_N][9];

int main() {
    int N; cin >> N;
    rep(i, 0, 9) dp[0][i] = 1;
    rep(i, 0, N) {
        rep(j, 0, 9) {
            dp[i+1][j] = dp[i][j];
            if (j > 0) dp[i+1][j] += dp[i][j-1];
            if (j < 8) dp[i+1][j] += dp[i][j+1];
            dp[i+1][j] %= mod;
        }
    }

    ll ans = 0;
    rep(i, 0, 9) ans = (ans + dp[N-1][i]) % mod;
    cout << ans << endl;
}
