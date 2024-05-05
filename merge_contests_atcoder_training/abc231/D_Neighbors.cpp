//             D - Neighbors
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc231/tasks/abc231_d
// ----------------------------------------

#include<bits/stdc++.h>
using namespace std;
#define rep(i, n) for(int i=0; i<(int)(n); i++)

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
  int n, m; cin >> n >> m;
  vector<int> dup(n);
  UnionFind uf(n);
  
  bool isOK = true;
  rep(i, m) {
    int a, b; cin >> a >> b;
    a--, b--;
    if (++dup[a]>2 || ++dup[b]>2) isOK = false;
    if (uf.issame(a, b)) isOK = false;
    uf.merge(a, b);
  }
  cout << (isOK ? "Yes" : "No") << endl;
}
