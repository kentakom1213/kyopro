//            Q2-1. 頂点を塗る
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/414
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define Yes(n) cout << ((n) ? "Yes" : "No"  ) << endl
typedef long long ll;
typedef vector<vector<int>> Graph;

int main() {
    int N, M;
    cin >> N >> M;

    Graph G(N);
    rep(i, N) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    // bfs
    
}