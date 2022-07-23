// https://atcoder.jp/contests/arc039/tasks/arc039_b

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int MAX_N=111, MAX_K=555;

int dp[MAX_N][MAX_K];

int main() {
    // 分割数配列の初期化
    dp[0][0] = 1;
    rep(n, 0, MAX_N) {
        rep(k, 1, MAX_K) {
            if (n - k >= 0) {
                dp[n][k] = dp[n][k-1] + dp[n-k][k];
            }
            else {
                dp[n][k] = dp[n][k-1];
            }
        }
    }

    // 入力
    int N, K; cin >> N >> K;

    // solve
    cout << dp[K][N] << endl;
    return 0;
}