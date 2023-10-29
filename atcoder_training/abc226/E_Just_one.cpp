//               E - Just one              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc226/tasks/abc226_e
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

typedef pair<int, int> P;
ll N, M;

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

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
    cin >> N >> M;
    
    vector<vector<int>> G(N, vector<int>());
    vector<P> edges(M);

    vector<ll> dim(N);  // 次数をカウント
    for (auto &[u, v] : edges) {
        cin >> u >> v;
        u--, v--;
        dim[u]++; dim[v]++;
        G[u].push_back(v);
        G[v].push_back(u);
    }

    // 次数1の頂点をキューに追加
    deque<int> q;
    rep(i, 0, N) {
        if (dim[i] == 0) {
            cout << 0 << endl;
            return 0;
        } else if (dim[i] == 1) {
            q.push_back(i);
        }
    }

    // ループになっていない頂点を削除していく
    while (!q.empty()) {
        int cur = q.front(); q.pop_front();
        dim[cur]--;
        for (int nxt : G[cur]) {
            if (dim[nxt] )dim[nxt]--;
            if (dim[nxt] == 1) q.push_back(nxt);
        }
    }

    // 全ての頂点の次数が0または2になっているか
    for (int d : dim) {
        if (d!=0 && d!=2) {
            cout << 0 << endl;
            return 0;
        }
    }

    // ループの個数をカウント
    UnionFind uf(N);
    for (auto [u, v] : edges) {
        if (dim[u]!=2 || dim[v]!=2) continue;
        uf.merge(u, v);
    }

    map<int, int> cnt;
    rep(i, 0, N) {
        int p = uf.root(i);
        cnt[p]++;
    }

    int ans = 1;
    for (auto [_, n] : cnt) {
        if (n >= 3) {
            ans *= 2;
            ans %= mod;
        }
    }

    cout << ans << endl;
}
