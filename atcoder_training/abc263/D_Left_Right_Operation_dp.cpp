//         D - Left Right Operation        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc263/tasks/abc263_d

// DP
// https://atcoder.jp/contests/abc263/editorial/4576
// https://atcoder.jp/contests/abc263/submissions/33900739
// ----------------------------------------

/*
# 更新
区間 -> {0:L, 1:A, 2:R}

dp[i+1][j] = min | dp[i+1][j]
                 | dp[i][k] + | L     (if j == 0)
                              | A[i]  (if j == 1)
                              | R     (if j == 2)
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
constexpr ll INF = (ll)1e18;

int main() {
    ll N, L, R; cin >> N >> L >> R;
    vector<ll> A(N);
    rep(i, 0, N) cin >> A[i];

    // dp[場所][1つ前の要素をどの区間に分類したか]
    Array<ll> dp(N+1, vector(3, INF));
    dp[0][0] = 0;

    rep(i, 0, N) {
        rep(j, 0, 3) {
            rep(k, 0, j + 1) {
                chmin(dp[i + 1][j],
                      dp[i][k] + (j == 0 ? L : (j == 1 ? A[i] : R)));
            }
        }
    }

    ll ans = *min_element(ALL(dp[N]));
    cout << ans << endl;
}
