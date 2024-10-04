//              E - Blackout 2             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc264/tasks/abc264_e
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

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

int main() {
    int N, M, E;
    cin >> N >> M >> E;
    vector<pii> path(E);
    for (auto &[u, v] : path) {
        cin >> u >> v;
        u--, v--;
    }
    int Q; cin >> Q;
    vector<int> queries(Q), q_sorted(Q), ans(Q);
    rep(i, 0, Q) {
        int q; cin >> q; q--;
        queries[i] = q;
        q_sorted[i] = q;
    }
    sort(ALL(q_sorted));  // ソート

    UnionFind uf(N+M);

    // 発電所を全て接続 -> issame(x, N) で接続判定ができる
    rep(i, N, N+M-1) {
        uf.merge(i, i+1);
    }

    // クエリに存在しないものを接続
    int j = 0;
    rep(i, 0, E) {
        if (q_sorted[j] == i) {
            j++;
        } else {
            auto [u, v] = path[i];
            uf.merge(u, v);
        }
    }

    // 逆順に処理
    for (int i = Q-1; i >= 0; i--) {
        ans[i] = uf.size(N) - M;
        // 接続
        auto [u, v] = path[queries[i]];
        uf.merge(u, v);
    }

    for (int a : ans) {
        cout << a << endl;
    }
}
