//           A - Many Formulae
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc122/tasks/arc122_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;

int main() {
    int N; cin >> N;
    int A[N];
    rep(i, N) cin >> A[i];

    int dp[100100][2];
    dp[1][0] = dp[1][1] = 1;
    range(i, 1, N+1) {
        dp[i+1][0] = dp[i][0] + dp[i][1] % MOD;
        dp[i+1][1] = dp[i][0];
    }

    ll ans = 0;
    rep(i, N) {
        ll K = 1;
        if (i == 1) {
            K = (dp[N-1][0] + dp[N-1][1]) % MOD;
            ans += A[i] * K % MOD;
            ans %= MOD;
        } else {
            K = dp[i][0];
            K *= dp[N-1-i][0] + dp[N-1-i][1];
            
        }
    }
}
