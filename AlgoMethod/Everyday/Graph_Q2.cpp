//             Q3-2. フォロー
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/412

// イテレータ便利

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M; cin >> N >> M;
    vector<set<int>> G(N);
    for (int i = 0; i < M; i++) {
        int a, b; cin >> a >> b;
        G[a].insert(b);
    }
    for (int i = 0; i < N; i++) {
        int W = G[i].size();
        for (auto itr = G[i].begin(); itr != G[i].end(); itr++) {
            cout << *itr << " ";
        }
        cout << endl;
    }
}