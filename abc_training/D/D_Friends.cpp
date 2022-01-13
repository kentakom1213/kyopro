//               D - Friends
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc177/tasks/abc177_d

// UnionFindは偉大

// AC
// ----------------------------------------

// グラフを作成
// 一番大きいグループの人数分に分割すればいい

// #include <bits/stdc++.h>
// using namespace std;
// typedef long long ll;

// int main() {
//     int N, M; cin >> N >> M;

//     vector<set<int>> G(N);
//     for (int i = 0; i < M; i++) {
//         int a, b; cin >> a >> b;
//         a--, b--;
//         G[a].insert(b);
//         G[b].insert(a);
//     }


//     // 共通の友達を持つ知り合いを友達に
// }

// やはり UnionFind か?

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

int main() {
    int N, M; cin >> N >> M;

    UnionFind friends(N);
    for (int i = 0; i < M; i++) {
        int a, b; cin >> a >> b;
        friends.merge(a-1, b-1);
    }

    map<int, int> community;
    for (int i = 0; i < N; i++) {
        community[friends.root(i)]++;
    }

    int max_com = 0;
    for (const auto [key, val] : community) {
        max_com = max(max_com, val);
    }

    cout << max_com << endl;
}