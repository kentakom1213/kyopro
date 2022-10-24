// https://atcoder.jp/contests/abc046/tasks/abc046_b

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N, K; cin >> N >> K;
    vector<ll> dp(1010, 0);
    dp[0] = K;

    for (int i = 0; i < N-1; i++) {
        dp[i+1] = dp[i] * (K-1);
    }

    cout << dp[N-1] << endl;
}
