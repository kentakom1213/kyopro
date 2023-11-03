//         039 - Tree Distance（★5）         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_am
// ----------------------------------------

/*
## 方針
- 部分木のサイズを求める
- 木dp

貢献度を subtree(X) * (N - subtree(X)) とする。
順番に足していく。
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
using Graph = vector<vector<int>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int N;                // グラフの頂点数
Graph G;              // グラフ
vector<ll> subtree;   // 部分木のサイズ
vector<ll> dp;        // 頂点iを通る最短経路

// 部分木をカウントする
void count_subtree(int cur, int prev) {
    subtree[cur] = 1;
    for (int nxt : G[cur]) {
        if (nxt == prev) continue;
        count_subtree(nxt, cur);
        subtree[cur] += subtree[nxt];
    }
}

// 最短経路を数え上げる
void dfs(int cur, int prev) {
    for (int nxt : G[cur]) {
        if (nxt == prev) continue;
        dfs(nxt, cur);
        dp[cur] += dp[nxt];
    }
    dp[cur] += subtree[cur] * (N - subtree[cur]);
}


int main() {
    cin >> N;
    G.assign(N, {});
    rep(i, 0, N-1) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    // 部分木サイズ
    subtree.assign(N, 0);
    count_subtree(0, -1);

    // dfsで数え上げ
    dp.assign(N, 0);
    dfs(0, -1);

    cout << dp[0] << endl;
}
