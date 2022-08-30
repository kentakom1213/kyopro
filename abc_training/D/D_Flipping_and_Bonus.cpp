//          D - Flipping and Bonus         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc261/tasks/abc261_d
// ----------------------------------------

/*
 * ## dpでとく
 * dp[i][j] := i回目のコイントスでカウンタがjであるときの価値の最大値
 * 
 * - ビットさんに感謝
 */

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
constexpr ll INF = numeric_limits<long long>::max();

int main() {
    int N, M; cin >> N >> M;
    vector<ll> X(N), CNT(N+1);
    rep(i, 0, N) cin >> X[i];
    rep(i, 0, M) {
        int c, y; cin >> c >> y;
        CNT[c] = y;
    }

    vector dp(N+10, vector<ll>(N+10, -INF));
    dp[0][0] = 0;

    // 更新
    rep(i, 0, N) {
        rep(j, 0, N+1) {
            // カウンタの数値を増やす
            chmax(
                dp[i+1][j+1],
                dp[i][j] + X[i] + CNT[j+1]
            );

            // カウンタの数値をリセット
            chmax(
                dp[i+1][0],
                dp[i][j]
            );
        }
    }

    cout << *max_element(ALL(dp[N])) << endl;
}
