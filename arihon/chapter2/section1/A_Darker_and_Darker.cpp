//          A - Darker and Darker          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/agc033/tasks/agc033_a
// ----------------------------------------

/*
## 方針
- 4近傍での移動を考えたとき、スタートから最大の距離となるマスの距離を考える
- BFS
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
const int MAX_SIZE = 1010;

typedef pair<int, int> P;
const P adj[] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

int H, W;
char A[MAX_SIZE][MAX_SIZE];
int dist[MAX_SIZE][MAX_SIZE];

int main() {
    cin >> H >> W;
    rep(i, H) rep(j, W) cin >> A[i][j];

    queue<P> q;

    // distの初期化
    rep(i, H) rep(j, W) {
        if (A[i][j] == '#') {
            dist[i][j] = 0;
            q.push({i, j});
        } else {
            dist[i][j] = -1;
        }
    }
    
    // BFS
    while (!q.empty()) {
        auto [r, c] = q.front(); q.pop();
        for (auto [dr, dc] : adj) {
            int nr=r+dr, nc=c+dc;
            if (0 <= nr && nr < H && 0 <= nc && nc < W && A[nr][nc]=='.' && dist[nr][nc]==-1) {
                dist[nr][nc] = dist[r][c] + 1;
                q.push({nr, nc});
            }
        }
    }

    int ans = 0;
    rep(i, H) {
        rep(j, W) {
            chmax(ans, dist[i][j]);
        }
    }

    cout << ans << endl;
}
