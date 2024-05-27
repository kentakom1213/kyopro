#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
using namespace atcoder;
#define rep(i, a, n) for(int i = a; i < n; i++)
#define rrep(i, a, n) for(int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

using mint = modint1000000007;
// using mint = modint998244353;
constexpr int IINF = 1001001001;
constexpr ll INF = 9e18;

template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int N;
vector<vector<int>> G;

// {白, 黒}
vector<vector<mint>> dp;

void rec(int u, int p) {
    dp[u] = {1, 1};

    // 葉の場合
    if (G[u].size() == 1 && G[u][0] == p) {
        return;
    }

    // 白に塗れる組合せ → 子のすべての塗り方の組合せ
    // 黒に塗れる組合せ → 子をすべて白く塗る組合せ
    for (auto v : G[u]) {
        if (v == p) {
            continue;
        }

        // 子の計算
        rec(v, u);

        // 白に塗る
        dp[u][0] *= dp[v][0] + dp[v][1];

        // 黒に塗る
        dp[u][1] *= dp[v][0];
    }
}

int main(){
    cin >> N;
    G.assign(N, {});

    rep(i, 0, N - 1) {
        int a, b;
        cin >> a >> b;
        a--; b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    dp.assign(N, {0, 0});

    // 木DP
    rec(0, -1);

    // 白に塗る組合せ + 黒に塗る組合せ
    mint ans = dp[0][0] + dp[0][1];

    cout << ans.val() << endl;

    return 0;
}
