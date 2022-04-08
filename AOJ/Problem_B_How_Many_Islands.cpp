//       Problem B: How Many Islands?      
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1160

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

void dfs(int N, int M, int r, int c, vector<vector<int> > &F) {
    F[r][c] = 0;

    for (int i=-1; i < 2; i++) {
        for (int j=-1; j<2; j++) {
            int nr=r+i, nc=c+j;
            if (0 <= nr && nr < N && 0 <= nc && nc < M && F[nr][nc]) {
                dfs(N, M, nr, nc, F);
            }
        }
    }
}

int main() {
    while (1) {
        int N, M; cin >> M >> N;
        if (N+M == 0) return 0;

        vector<vector<int> > F(N, vector(M, 0));
        rep(i, N) rep(j, M) cin >> F[i][j];

        int ans = 0;
        rep(i, N) {
            rep(j, M) {
                if (F[i][j]) {
                    ans++;
                    dfs(N, M, i, j, F);
                }
            }
        }

        cout << ans << endl;
    }
}