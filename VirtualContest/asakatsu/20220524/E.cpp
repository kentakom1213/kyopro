// https://atcoder.jp/contests/abc212/tasks/abc212_e

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

bool G[5050][5050];
ll dp[5050][5050];

int main() {
    int N, M, K;
    cin >> N >> M >> K;
    rep(i, 0, M) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a][b] = 1, G[b][a] = 1;
    }

    dp[0][0] = 1;

    // 愚直なDP
    rep(k, 0, K) {
        rep(i, 0, N) rep(j, i+1, N) {
            if (G[i][j]) continue;
            dp[k+1][i] = (dp[k+1][i] + dp[k][j]) % mod;
            dp[k+1][j] = (dp[k+1][j] + dp[k][i]) % mod;
        }
    }

    cout << dp[K][0] << endl;
}
