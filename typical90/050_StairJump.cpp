//         050 - Stair Jump（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ax
// ----------------------------------------

// 1段かL段
// dpテーブルを作って解いてみよう

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr ll MOD = 1000000007;

int main() {
    ll N, L; cin >> N >> L;
    vector<ll> dp(N+1, 0);
    dp[0] = 1;

    rep(i, N) {
        dp[i+1] = (dp[i] + dp[i+1]) % MOD;
        if (i+L <= N) {
            dp[i+L] = (dp[i] + dp[i+L]) % MOD;
        }
    }

    cout << dp[N] << endl;
}