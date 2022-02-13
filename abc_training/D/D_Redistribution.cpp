//           D - Restribution
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc178/tasks/abc178_d

// 難しい、要復習

// AC (解説)
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr ll MOD = 1000000007;

int main() {
    int S; cin >> S;
    vector<ll> dp(S+1);
    dp[0] = 1, dp[1] = dp[2] = 0;
    for (int i=3; i <= S; i++) {
        dp[i] = (dp[i-1] + dp[i-3]) % MOD;
    }
    cout << dp[S] << endl;
}
