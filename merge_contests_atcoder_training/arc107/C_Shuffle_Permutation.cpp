//         C - Shuffle Permutation         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc107/tasks/arc107_c
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

ll A[55][55];

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

template<class T, class S>
ll factorial_mod(T n, S MOD) {
    if (n == 0) return 1;

    ll ans = 1;
    for (ll i=1; i <= n; i++) {
        ans = ans * i % MOD;
    }
    return ans;
}

int main() {
    ll N, K; cin >> N >> K;
    rep(i, 0, N) rep(j, 0, N) cin >> A[i][j];

    UnionFind row(N), col(N);

    // (row[x], row[y]), (col[x], col[y]) が交換可能か判定
    rep(x, 0, N) {
        rep(y, x, N) {
            bool isOK_r=true, isOK_c=true;
            rep (i, 0, N) {
                if (A[x][i] + A[y][i] > K) isOK_r = false;
                if (A[i][x] + A[i][y] > K) isOK_c = false;
            }
        if (isOK_r) row.merge(x, y);
        if (isOK_c) col.merge(x, y);
        }
    }

    // グループの大きさを取得
    map<int, ll> grow, gcol;
    rep(i, 0, N) {
        grow[row.root(i)]++;
        gcol[col.root(i)]++;
    }

    // 行、列それぞれに組み合わせを計算
    ll row_res=1, col_res=1;
    for (auto [_, r] : grow) {
        row_res = row_res * factorial_mod(r, mod) % mod;
    }
    for (auto [_, c] : gcol) {
        col_res = col_res * factorial_mod(c, mod) % mod;
    }

    cout << (row_res * col_res % mod) << endl;
}
