
// 累積和をとってDP？
// 時間があれば行けたな

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

using mint = modint<mod>;

mint dp[1010][5050];

int main() {
    int N, M, K;
    cin >> N >> M >> K;

    rep(i, 0, M) {
        dp[0][i] = 1;
    }

    rep(i, 0, N) {
        rep(j, 0, K) {
            dp[i]
        }
    }
}
