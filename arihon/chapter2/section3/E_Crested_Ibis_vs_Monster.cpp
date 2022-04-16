//       E - Crested Ibis vs Monster       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc153/tasks/abc153_e

// AC
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
const int INF = 1 << 30;

const int MAX_H = 20000;
int H, N;
int dp[1010][MAX_H];
// dp[i][j] := i番目までの魔法を使い、jのダメージを与えるときの魔力の最小値

int main() {
    FILL(dp, INF);
    dp[0][0] = 0;

    cin >> H >> N;
    for (int i=0; i < N; i++) {
        int a, b; cin >> a >> b;
        for (int j=0; j <= MAX_H; j++) {
            chmin(dp[i+1][j], dp[i][j]);
            if (a <= j) {
                chmin(dp[i+1][j], dp[i+1][j-a]+b);
            }
        }
    }

    int ans = INF;
    for (int i=H; i<MAX_H; i++) {
        chmin(ans, dp[N][i]);
    }

    cout << ans << endl;
}
