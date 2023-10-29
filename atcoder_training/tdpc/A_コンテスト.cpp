//                A - コンテスト                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/tdpc/tasks/tdpc_contest
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

const int MAX_N=111, MAX_SIZE=10101;
int N, dp[MAX_N][MAX_SIZE];

int main() {
    cin >> N;

    dp[0][0] = 1;

    rep(i, N) {
        int p; cin >> p;
        rep(j, MAX_SIZE) {
            if (dp[i][j] == 0) continue;
            dp[i+1][j] = 1;
            if (j+p < MAX_SIZE) {
                dp[i+1][j+p] = 1;
            }
        }
    }

    int ans = 0;
    rep(i, MAX_SIZE) ans += dp[N][i];

    cout << ans << endl;
}
