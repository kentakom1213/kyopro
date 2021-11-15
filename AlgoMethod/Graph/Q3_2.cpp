//              Q3-2. 頂点の深さ
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/529
// ----------------------------------------

#include <iostream>
#include <vector>
using namespace std;

vector<vector<int>> G;
vector<int> depth;

void dfs(int now, int now_depth = 0) {
    if (depth[now] != -1) return;

    depth[now] = now_depth;
    for (int next : G[now]) {
        dfs(next, now_depth + 1);
    }
}

int main() {
    int N; cin >> N;
    G.assign(N, vector<int>());
    for (int i = 1; i < N; i++) {
        int p; cin >> p;
        G[p].push_back(i);
    }

    depth.assign(N, -1);
    dfs(0);

    for (int i = 0; i < N; i++) {
        cout << depth[i] << " ";
    }
    cout << endl;
}