// https://atcoder.jp/contests/abc209/tasks/abc209_d

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


void dfs(int cur, vector<vector<int>> &G, vector<int> &dist) {
    for (int nxt : G[cur]) {
        if (dist[nxt] != -1) continue;
        dist[nxt] = dist[cur] + 1;
        dfs(nxt, G, dist);
    }
}

int main() {
    int N, Q; cin >> N >> Q;

    vector<vector<int>> G(N);
    rep(i, 0, N-1) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    // 頂点0からの距離を表す配列
    vector<int> dist(N, -1);
    dist[0] = 0;
    dfs(0, G, dist);

    while (Q--) {
        int a, b; cin >> a >> b;
        int d = dist[a-1] + dist[b-1];
        cout << (d & 1 ? "Road" : "Town") << endl;
    }
}