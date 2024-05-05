//             C - Simple path             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc270/tasks/abc270_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

vector<vector<int>> G;

int main() {
    int N, X, Y;
    cin >> N >> X >> Y;
    X--, Y--;

    G.assign(N, {});
    for (int i=0; i<N-1; i++) {
        int u, v; cin >> u >> v;
        u--, v--;
        G[u].emplace_back(v);
        G[v].emplace_back(u);
    }

    
}
