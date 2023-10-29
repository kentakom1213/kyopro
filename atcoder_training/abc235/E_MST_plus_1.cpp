//              E - MST + 1
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc235/tasks/abc235_e
// ----------------------------------------

// 愚直にクラスカル法: O(NlogN * Q)
// setで辺を管理しても O(NQ)

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

// Union-Find  (引用元: https://drken1215.hatenablog.com/entry/2021/07/28/014400)
struct UnionFind {
    vector<int> par;

    UnionFind() { }
    UnionFind(int n) : par(n, -1) { }
    void init(int n) { par.assign(n, -1); }
    
    int root(int x) {
        if (par[x] < 0) return x;
        else return par[x] = root(par[x]);
    }
    
    bool issame(int x, int y) {
        return root(x) == root(y);
    }
    
    bool merge(int x, int y) {
        x = root(x); y = root(y);
        if (x == y) return false;
        if (par[x] > par[y]) swap(x, y); // merge technique
        par[x] += par[y];
        par[y] = x;
        return true;
    }
    
    int size(int x) {
        return -par[root(x)];
    }
};

//             w    u    v    i
typedef tuple<int, int, int, int> edge;

int main() {
    int N, M, Q; cin >> N >> M >> Q;
    vector<edge> edges(M+Q);

    rep(i, M+Q) {
        auto& [w, u, v, ei] = edges[i];
        cin >> u >> v >> w;
        ei = i >= M ? i-M : -1;
    }
    sort(ALL(edges));

    vector<int> ans(Q, 0);

    // クラスカル法
    UnionFind uf(N);
    for (auto [w, u, v, ei] : edges) {
        if (uf.issame(u, v)) continue;
        if (ei == -1) {
            uf.merge(u, v);
        } else {
            ans[ei]++;
        }
    }

    // クエリの処理
    rep(i, Q) {
        cout << (ans[i] ? "Yes" : "No") << endl;
    }
}
