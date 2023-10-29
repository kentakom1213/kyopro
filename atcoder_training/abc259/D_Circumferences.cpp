//            D - Circumferences           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc259/tasks/abc259_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
using Circle = tuple<ll, ll, ll>;
#define sq(a) ((a) * (a))

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

bool is_touching(Circle a, Circle b) {
    auto [x1, y1, r1] = a;
    auto [x2, y2, r2] = b;
    
    // d := sqrt((x1 - x2)^2 + (y1 - y2)^2)
    //     |r1 - r2| <= d <= r1 + r2
    // <-> (r1 - r2)^2 <= d^2 <= (r1 + r2)^2

    ll d_sq = sq(x1 - x2) + sq(y1 - y2);
    return sq(r1 - r2) <= d_sq && d_sq <= sq(r1 + r2);
}

int main() {
    int N; cin >> N;
    ll sx, sy, tx, ty;
    cin >> sx >> sy >> tx >> ty;
    int all=N+2, start=N, end=N+1;

    UnionFind uf(all);
    vector<Circle> circles(all);  // start, endを円として保存

    rep(i, 0, N) {
        auto &[x, y, r] = circles[i];
        cin >> x >> y >> r;
    }

    circles[start] = {sx, sy, 0};
    circles[end] = {tx, ty, 0};

    // O(N^2) で全ての円同士で接触判定
    rep(i, 0, all) {
        rep(j, i, all) {
            if (is_touching(circles[i], circles[j])) {
                uf.merge(i, j);
            }
        }
    }

    // start, endの連結判定
    bool isOK = uf.issame(start, end);
    cout << (isOK ? "Yes" : "No") << endl;
}
