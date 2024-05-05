//                  D - 派閥                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc002/tasks/abc002_4
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

int main() {
    int N, M; cin >> N >> M;
    vector<vector<int>> G(N, vector(N, 0));
    for (int i=0; i < M; i++) {
        int x, y; cin >> x >> y;
        x--, y--;
        G[x][y] = G[y][x] = 1;
    }

    // N <= 12 であるから、部分集合全てを考える
    int ans = 0;
    for (int i = 0; i < (1<<N); i++) {
        bool isOK = true;
        for (int u = 0; u < N; u++) {
            for (int v = u+1; v < N; v++) {
                if (((i>>u)&1) && ((i>>v)&1)) isOK &= G[u][v];
            }
        }

        if (isOK) {
            int tmp = 0;
            for (int j = 0; j < 20; j++) {
                tmp += (i >> j) & 1;
            }
            chmax(ans, tmp);
        }
    }

    cout << ans << endl;
}
