//            D - Even Relation            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc126/tasks/abc126_d
// ----------------------------------------

/*
## 方針
- 根からの距離が偶数の頂点を0，奇数の頂点を1と塗る
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
template<typename T> using Array = vector<vector<T>>;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int N_MAX = 101010;

Array<pii> G;
int dist[N_MAX], color[N_MAX];

void dfs(int cur) {
    for (auto [nxt, w] : G[cur]) {
        if (dist[nxt] != -1) continue;
        dist[nxt] = dist[cur] + w;
        color[nxt] = dist[nxt] & 1;
        dfs(nxt);
    }
}

int main() {
    int N; cin >> N;
    G.assign(N, {});
    rep(i, 0, N-1) {
        int u, v, w; cin >> u >> v >> w;
        u--, v--;
        G[u].push_back({v, w});
        G[v].push_back({u, w});
    }

    FILL(dist, -1);
    dist[0] = 0;

    dfs(0);

    rep(i, 0, N) cout << color[i] << "\n";
}