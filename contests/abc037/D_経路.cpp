//                  D - 経路                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc037/tasks/abc037_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int AROUND[] = {0, 1, 0, -1, 0};
int H, W, A[1010][1010], dp[1010][1010];

ll dfs(int r, int c) {
    if (dp[r][c]) return dp[r][c];

    ll res = 0;
    rep(i, 0, 4) {
        int nr=r+AROUND[i], nc=c+AROUND[i+1];
        if (0 <= nr && nr < H && 0 <= nc && nc < W && A[r][c] < A[nr][nc]) {
            res += dfs(nr, nc) + 1;
            res %= MOD;
        }
    }
    return dp[r][c] = res;
}

int main() {
    cin >> H >> W;
    rep(i, 0, H) rep(j, 0, W) scanf("%d", &A[i][j]);

    ll ans = 0;
    rep(i, 0, H) rep(j, 0, W) {
        ans = (ans + dfs(i, j) + 1) % MOD;
    }

    cout << ans << endl;
}
