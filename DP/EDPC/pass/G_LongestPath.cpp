//            G - Longest Path
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_g

// 参考
// https://qiita.com/drken/items/03c7db44ccd27820ea0d

// 探索順序がわからない場合はメモ化再帰を使おう
// よくわからん！

// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmax(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int N, M;
vector<vector<int>> G; // グラフ

int dp[100100];
int rec(int v) {
    if (dp[v] != -1) return dp[v]; // 更新済みの場合

    int res = 0;
    for (auto nv : G[v]) {
        chmax(res, rec(nv) + 1);
    }
    return dp[v] = res;
}

int main() {
    int N, M; cin >> N >> M;
    G.assign(N, vector<int>());
    for (int i = 0; i < M; i++) {
        int x, y; cin >> x >> y;
        x--, y--;
        G[x].push_back(y);
    }

    // dp配列の初期化
    for (int v = 0; v < N; v++) dp[v] = -1;

    // 全ノードを更新しながら答えを求める
    int res = 0;
    for (int v = 0; v < N; v++) chmax(res, rec(v));
    cout << res << endl;
}

