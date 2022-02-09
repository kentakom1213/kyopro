//       D - Sum of Maximum Weights
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc214/tasks/abc214_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, ll> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

ll ans = 0;
vector<int> is_visited;
vector<vector<vec2>> G;

void dfs(int cur, ll max_w, int depth) {
    is_visited[cur] = 1;
    for (auto [nxt, w] : G[cur]) {
        if (is_visited[nxt]) continue;
        chmax(max_w, w);
        ans += max_w * depth;
        dfs(nxt, max_w, depth+1);
        cerr << cur << " -> " << nxt << ": " << max_w << " * " << depth << endl;
    }
}

int main() {
    int N; cin >> N;
    G.assign(N, vector<vec2>());
    rep(i, N-1) {
        int u, v; cin >> u >> v;
        u--, v--;
        ll w; cin >> w;
        G[u].push_back(make_pair(v, w));
        G[v].push_back(make_pair(u, w));
    }

    is_visited.assign(N, 0);
    dfs(0, 0, 1);
    cout << ans << endl;
}
