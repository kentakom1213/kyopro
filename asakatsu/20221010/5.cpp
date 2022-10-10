// https://atcoder.jp/contests/abc126/tasks/abc126_d

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

void dfs(int cur, const vector<vector<pll>> &G, vector<int> &dist) {
    for (auto [nxt, w] : G[cur]) {
        if (dist[nxt] != -1) continue;
        dist[nxt] = dist[cur] + w;
        dfs(nxt, G, dist);
    }
}

int main() {
    int N; cin >> N;
    vector<vector<pll>> G(N, vector<pll>());
    for (int i = 0; i < N-1; i++) {
        int u, v, w; cin >> u >> v >> w;
        u--, v--;
        G[u].emplace_back(v, w);
        G[v].emplace_back(u, w);
    }

    // 距離を求める
    vector<int> dist(N, -1);
    dist[0] = 0;
    dfs(0, G, dist);

    // 偶数なら0, 奇数なら1に塗る
    for (int i = 0; i < N; i++) {
        if (dist[i] & 1) {
            cout << 1 << endl;
        } else {
            cout << 0 << endl;
        }
    }
}
