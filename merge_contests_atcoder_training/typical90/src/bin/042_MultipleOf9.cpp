//         042 - Multiple of 9（★4）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ap
// ----------------------------------------

// 9の倍数の各桁を足した和は9の倍数になる
// 1桁の整数の和に分解する組み合わせを求めれば良い

// 例)
// 9 = 1+1+1+1+1+1+1+1
//   = 1+1+1+1+1+1+2
//   = 1+1+1+1+1+2+1
//   = 1+1+1+1+1+3
//   ...
//   = 8+1
//   = 9
// 
// 18 = 1+  ...  +1
//    = 1+ ... +2
//    = 1+...+2+1
//    ...
//    = 9+1+8
//    = 9+8+1
//    = 9+9

// ## 解説
// 重複組合せにはdpを使う
// dp[各桁の数字の和] = 通り数


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
constexpr long long MOD = 1000000007;

int main() {
    ll K; cin >> K;
    if (K % 9) {
        cout << 0 << endl;
        return 0;
    }

    vector<ll> dp(K+10, 0);
    dp[0] = 1;
    for (int i = 1; i <= K; i++) {
        int range = min(i, 9);
        // dp[i] = sum(j=1)(9) dp[i-j]
        for (int j = 1; j <= range; j++) {
            dp[i] = (dp[i] + dp[i - j]) % MOD;
        }
    }

    cout << dp[K] << endl;
}