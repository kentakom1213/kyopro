//             Knapsack Problem            
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_C
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

int n, W;
ll dp[111][10101];  // 配列[行][列];

int main() {
    cin >> n >> W;
    for (int i=0; i<n; i++) {
        ll v, w; cin >> v >> w;
        for (int j=0; j<=W; j++) {
            if (j < w) {
                dp[i+1][j] = dp[i][j];
            } else {
                dp[i+1][j] = max(
                    dp[i][j],
                    dp[i+1][j-w] + v
                );
            }
        }
    }
    cout << dp[n][W] << endl;
}
