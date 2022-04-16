//             Knapsack Problem            
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_C
// ----------------------------------------

/*
- dp配列の再利用を学ぶ
*/

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

int N, W;
int dp[10101];

int main() {
    cin >> N >> W;
    rep(i, N) {
        int v, w; cin >> v >> w;
        for (int j=w; j<=W; j++) {
            dp[j] = max(dp[j], dp[j-w]+v);
        }
    }
    cout << dp[W] << endl;
}
