//            Q3-1. 行きがけ順
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/525
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
    int N; cin >> N;
    vector<vector<int>> G(N, vector<int>());
    for (int i = 1; i < N; i++) {
        int p; cin >> p;
        G[p].push_back(i);
    }

    // 最近のトレンドは非再帰DFS
    stack<int> points;
    points.push(0);
    while (!points.empty()) {
        int next_point = points.top();
        points.pop();
        cout << next_point << endl;
        
        while (!G[next_point].empty()) {
            points.push(G[next_point].back());
            G[next_point].pop_back();
        }
    }
}