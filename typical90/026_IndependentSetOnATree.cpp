//   026 - Independent Set on a Tree（★4）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_z

// ----------------------------------------

// 解説
// https://kaage.hatenablog.com/entry/2021/05/04/144829

// 2部グラフという概念が必要らしい
// | 変で直接結ばれた頂点同士が互いに違う色になるように塗ることができるグラフ

// 木は必ず2部グラフになるから、depthが大きい順に出力すれば良い


// ----- 謎のエラー -----

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

vector<int> depth;  // 深さ、-1の場合は探索していない

void dfs(vector<vector<int>> &G, int now_point, int now_depth) {
    // 探索済みの場合
    if (depth[now_point] != -1) return;

    // 探索済みでない場合
    depth[now_point] = now_depth + 1;
    for (auto next_point : G[now_point]) {
        dfs(G, next_point, now_depth + 1);
    }
}

int main() {
    int N; cin >> N;  // 頂点の数
    vector<vector<int>> G;  // グラフ

    depth.assign(N, -1);

    for (int i = 0; i < N-1; i++) {
        int a, b; cin >> a >> b;
        a--, b--;
        printf("a:%d, b:%d\n", a, b);
        G[a].push_back(b);
        G[b].push_back(a);
    }

    dfs(G, 0, 0);
}