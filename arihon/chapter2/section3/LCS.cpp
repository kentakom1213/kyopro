/*
# [Longest Common Subsequence](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_C&lang=jp)

# AC
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

int dp[1010][1010];

int main() {
    int q; cin >> q;
    while (q--) {
        string x, y; cin >> x >> y;
        FILL(dp, 0);
        rep(i, x.size()) {
            rep(j, y.size()) {
                dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1]);
                if (x[i] == y[j]) {
                    dp[i+1][j+1] = dp[i][j] + 1;
                }
            }
        }
        cout << dp[x.size()][y.size()] << endl;
    }
}
