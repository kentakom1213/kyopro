//                C - 幅優先探索                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc007/tasks/abc007_3
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
typedef pair<int, int> P;

const int F_MAX = 100;
constexpr P adj[] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

int R, C, sr, sc, gr, gc;
char F[F_MAX][F_MAX];
int dist[F_MAX][F_MAX];

int main() {
    cin >> R >> C >> sr >> sc >> gr >> gc;
    sr--, sc--, gr--, gc--;

    rep(i, R) rep(j, C) cin >> F[i][j];

    // distの初期化
    rep(i, R) rep(j, C) dist[i][j] = -1;
    dist[sr][sc] = 0;
    
    // 幅優先探索
    queue<P> q;
    q.push(make_pair(sr, sc));
    while (!q.empty()) {
        auto [cr, cc] = q.front(); q.pop();
        for (auto [dr, dc] : adj) {
            int nr=cr+dr, nc=cc+dc;
            if (0 <= nr && nr < R && 0 <= nc && nc < C && F[nr][nc]=='.' && dist[nr][nc]==-1) {
                dist[nr][nc] = dist[cr][cc] + 1;
                q.push(make_pair(nr, nc));
            }
        }
    }

    cout << dist[gr][gc] << endl;
}
