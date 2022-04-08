/*
# Lake Counting (POJ No.2386)

## 制約
- $N,M \le 100$

## 方針

*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int N, M;
char F[110][110];

void dfs(int r, int c) {
    F[r][c] = '.';

    for (int i=-1; i < 2; i++) {
        for (int j=-1; j<2; j++) {
            int nr=r+i, nc=c+j;
            if (0 <= nr && nr < N && 0 <= nc && nc < M && F[nr][nc] == 'W') {
                dfs(nr, nc);
            }
        }
    }
}

int main() {
    cin >> N >> M;
    rep(i, N) rep(j, M) cin >> F[i][j];

    int ans = 0;
    rep(i, N) {
        rep(j, M) {
            if (F[i][j] == 'W') {
                ans++;
                dfs(i, j);
            }
        }
    }

    cout << ans << endl;
}