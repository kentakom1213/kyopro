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

int N, M, K;
int dp[55][3000];

int main() {
    cin >> N >> M >> K;
    dp[0][0] = 1;

    rep(i, 0, N) {
        rep(j, 0, K) {
            rep(k, 1, M+1) {
                dp[i+1][j+k] = (dp[i+1][j+k] + dp[i][j]) % mod;
            }
        }
    }

    int ans = 0;
    rep(i, 1, K+1) {
        ans = (ans + dp[N][i]) % mod;
    }

    cout << ans << endl;
}