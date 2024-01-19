//           E - Subsequence Path          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc271/tasks/abc271_e
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
constexpr ll INF = 1LL << 60;

int main() {
    int N, M, K; cin >> N >> M >> K;
    vector<array<int, 3>> path(M), E(K);
    for (auto &[a, b, c] : path) {
        cin >> a >> b >> c;
        a--, b--;
    }

    // DP
    vector<ll> dp(N, INF);
    dp[0] = 0;

    for (int i = 0; i < K; i++) {
        int e; cin >> e;
        auto [u, v, w] = path[e-1];
        chmin(dp[v], dp[u] + w);
    }

    if (dp[N-1] >= INF) {
        cout << -1 << endl;
    } else {
        cout << dp[N-1] << endl;
    }
}
