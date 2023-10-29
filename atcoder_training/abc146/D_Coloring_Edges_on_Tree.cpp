//        D - Coloring Edges on Tree       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc146/tasks/abc146_d
// ----------------------------------------

/*
- 根付き木を用いて貪欲に
*/

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

using pii = pair<int, int>;
vector<vector<int>> G;

int main() {
    int N; cin >> N;
    G.assign(N, {});
    vector<pii> edges(N-1);
    map<pii, int> I;

    rep(i, 0, N-1) {
        int a, b; cin >> a >> b;
        a--, b--;
        edges[i] = {a, b};
        G[a].push_back(b);
        G[b].push_back(a);
    }

    // bfs
    int K = 0;  // 次数の最大値を保存
    vector<int> cs(N, 0), used(N, 0);
    used[0] = 1;
    queue<int> q({0});
    while (!q.empty()) {
        int cur = q.front(); q.pop();
        chmax(K, (int)G[cur].size());
        int col = 1;
        for (int nxt : G[cur]) {
            if (used[nxt]) continue;
            if (col == cs[cur]) col++;
            cs[nxt] = I[{cur, nxt}] = I[{nxt, cur}] = col++;
            used[cur] = 1;
            q.push(nxt);
        }
    }

    cout << K << endl;
    for (auto edge : edges) cout << I[edge] << endl;
}
