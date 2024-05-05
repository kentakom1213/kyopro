//    D - Index × A(Not Continuous ver.)   
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc267/tasks/abc267_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
constexpr ll INF = (ll)1e18;

int main() {
    int N, M; cin >> N >> M;
    vector<int> A(N);
    rep(i, 0, N) cin >> A[i];

    // dp
    vector<vector<ll>> dp(N+1, vector(M+1, -INF));
    rep(i, 0, M+1) dp[0][i] = 0;
    rep(i, 0, N+1) dp[i][0] = 0;

    for (int i = 1; i <= N; i++) {
        for (int j = 1; j <= M; j++) {
            // 選ばないとき
            if (i > j) {
                chmax(dp[i][j], dp[i-1][j]);
            }
            // 選ぶとき
            chmax(dp[i][j], dp[i-1][j-1] + A[i-1] * j);
        }
    }

    cout << dp[N][M] << endl;
}
