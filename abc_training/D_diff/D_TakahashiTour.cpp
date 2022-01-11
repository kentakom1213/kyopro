// D - Takahashi Tour
// ------------------
// 問題
// https://atcoder.jp/contests/abc213/tasks/abc213_d
//
// AC
// ------------------

// 普通のdfsかな
// ちょっとちがうっぽい

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int> > Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

void dfs(Array &G, vector<int> &is_visited,  int cur) {
    cout << cur+1 << " ";
    is_visited[cur] = 1;
    for (auto nxt : G[cur]) {
        if (is_visited[nxt]) {
            continue;
        }
        dfs(G, is_visited, nxt);
        cout << cur+1 << " ";
    }
}

int main() {
    int N; cin >> N;
    Array G(N);
    rep(i, N-1) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }
    rep(i, N) sort(ALL(G[i]));
    vector<int> is_visited(N, 0);

    dfs(G, is_visited, 0);
    cout << endl;
}
