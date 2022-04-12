//            D - 高橋くんの苦悩               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc015/tasks/abc015_4

// AC
// ----------------------------------------

/*
## 方針
- dp[i][j][w] := i個めまででj個選び、幅がwであるときの価値の最大値
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

const int MAX_N=55, MAX_W=10101;
int W, N, K;
ll A[MAX_N], B[MAX_N];
ll dp[MAX_N][MAX_N][MAX_W];

int main() {
    cin >> W >> N >> K;
    rep(i, N) cin >> A[i] >> B[i];

    FILL(dp, -1);
    dp[0][0][0] = 0;

    rep(i, N) {
        rep(j, K+1) {
            rep(w, W+1) {
                // 使わない
                chmax(
                    dp[i+1][j][w],
                    dp[i][j][w]
                );
                // 使う
                chmax(
                    dp[i+1][j+1][w+A[i]],
                    dp[i][j][w] + B[i]
                );
            }
        }
    }

    ll ans = 0;
    rep(j, K+1) {
        rep(w, W+1) {
            chmax(ans, dp[N][j][w]);
        }
    }

    cout << ans << endl;
}
