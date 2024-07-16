#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, n) for (int i = a; i < n; i++)
#define rrep(i, a, n) for (int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

constexpr int IINF = 1001001001;
constexpr ll INF = 9e18;

template <class t, class u>
void chmax(t& a, u b) {
    if (a < b) a = b;
}
template <class t, class u>
void chmin(t& a, u b) {
    if (b < a) a = b;
}

int main() {
    while (1) {
        int n;
        cin >> n;
        if (n == 0) return 0;

        vector<int> B(n);
        rep(i, 0, n) cin >> B[i];

        vector dp(n + 1, vector(2501, vector<bool>(2501, false)));

        dp[0][0][0] = true;

        int S = 0;

        rep(i, 0, n) {
            S += B[i];
            rep(j, 0, S + 1) {
                rep(k, 0, S + 1) {
                    if (dp[i][j][k] && j + k <= S - B[i]) {
                        dp[i + 1][j + B[i]][k] = true;
                        dp[i + 1][j][k + B[i]] = true;
                        dp[i + 1][j][k] = true;
                    }
                }
            }
        }

        // 最小値が最も小さい値を取得
        int ans = 0;

        rep(j, 0, S + 1) {
            rep(k, 0, S + 1) {
                if (dp[n][j][k] && j + k <= S) {
                    ans = max(ans, min({j, k, S - j - k}));
                }
            }
        }

        cout << ans << endl;
    }
}
