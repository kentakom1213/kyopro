//            Q3-3. 友達の友達
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/413

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M, X; cin >> N >> M >> X;
    vector<set<int>> G(N);
    for (int i = 0; i < M; i++) {
        int a,b; cin >> a >> b;
        G[a].insert(b);
        G[b].insert(a);
    }

    // 友達を列挙
    set<int> friends;
    for (auto v : G[X]) friends.insert(v);

    // 友達の友達を列挙
    set<int> result;
    for (auto v : G[X]) {
        for (auto w : G[v]) {
            // アルル自身と友達を排除
            if (w != X && !friends.count(w)) {
                result.insert(w);
            }
        }
    }


    cout << result.size() << endl;
}