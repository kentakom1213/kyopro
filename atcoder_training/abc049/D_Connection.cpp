//                D - 連結
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc049/tasks/arc065_b
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

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


// 2つのUnionFind木を作り、相互関係を調べる

int main() {
    int N, K, L; cin >> N >> K >> L;

    UnionFind road(N), train(N);
    for (int i = 0; i < K; i++) {
        int p, q; cin >> p >> q;
        p--, q--;
        road.merge(p, q);
    }
    for (int i = 0; i < L; i++) {
        int r, s; cin >> r >> s;
        r--, s--;
        train.merge(r, s);
    }

    map<pair<int, int>, int> nums;
    for (int i = 0; i < N; i++) {
        int root_road = road.root(i);
        int root_train = train.root(i);

        nums[make_pair(root_road, root_train)]++;
    }

    // 出力
    for (int i = 0; i < N; i++) {
        int root_road = road.root(i);
        int root_train = train.root(i);
        cout << nums[make_pair(root_road, root_train)] << " ";
    }
    cout << endl;
}