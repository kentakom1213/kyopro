//                 B - 埋め立て                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc031/tasks/arc031_2
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

char F[10][10];
vector<pair<int, int> > adj = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

void dfs(int r, int c, vector<vector<int> > &is_visited, int &size) {
    size++;
    is_visited[r][c] = 1;
    for (auto [dr, dc] : adj) {
        int nr=r+dr, nc=c+dc;
        if (0 <= nr && nr < 10 && 0 <= nc && nc < 10 && F[nr][nc]=='o' && is_visited[nr][nc]==0) {
            dfs(nr, nc, is_visited, size);
        }
    }
}

int main() {
    int cntO = 0;
    rep(i, 10) rep(j, 10) {
        char c; cin >> c;
        if (c == 'o') cntO++;
        F[i][j] = c;
    }

    // 1マスずつ埋め立てながらdfs
    bool isOK = false;
    rep(i, 10) {
        rep(j, 10) {
            vector<vector<int> > is_visited(10, vector(10, 0));
            int size_of_island = 0;

            char tmp = F[i][j];
            F[i][j] = 'o';
            dfs(i, j, is_visited, size_of_island);
            F[i][j] = tmp;

            cerr << size_of_island << endl;

            if (size_of_island > cntO) isOK = true;
        }
    }

    cout << (isOK ? "YES" : "NO") << endl;
}
