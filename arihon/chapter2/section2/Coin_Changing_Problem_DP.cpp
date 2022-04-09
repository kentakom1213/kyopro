//          Coin Changing Problem          
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_A
// ----------------------------------------

/*
## 制約
- n <= 50000
- m <= 20
- よって、$O(nm)$ までなら通せる

## 方針
- DP
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int INF = 1 << 28;
const int MAX_PAY = 50100;
int dp[MAX_PAY];

int main() {
    int N, M; cin >> N >> M;
    vector<int> coins(M);
    rep(i, M) cin >> coins[i];

    // dp配列の初期化
    rep(i, MAX_PAY) dp[i] = INF;
    dp[0] = 0;

    rep(i, MAX_PAY) {
        for (int x : coins) {
            if (i+x < MAX_PAY) {
                chmin(dp[i+x], dp[i]+1);
            }
        }
    }

    cout << dp[N] << endl;
}
