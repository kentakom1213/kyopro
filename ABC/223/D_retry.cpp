

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

vector<int> topological_sort(vector<vector<int>>& G, vector<int>& indegree, int V) {
    // 記録用
    vector<int> sorted;

    // 次数0の頂点をキューに追加
    priority_queue<int, vector<int>, greater<int>> que;
    for (unsigned int i = 0; i < V; i++) {
        if (indegree[i] == 0) que.push(i);
    }

    // キューが空になるまで繰り返す
    while (que.empty() == false) {
        // キューの先頭の頂点を取り出す
        int v = que.top();
        que.pop();

        // その頂点と隣接している頂点の入次数を減らす
        for (unsigned int i = 0; i < G[v].size(); i++) {
            int u = G[v][i];
            indegree[u]--;
            // 0になったら追加
            if (indegree[u] == 0) que.push(u);
        }

        // 頂点vを配列の末尾に追加
        sorted.push_back(v);
    }

    return sorted;
}



int main() {
    int N, M; cin >> N >> M;

    // トポロジカルソート？？？
    // N: 頂点, M: 辺
    vector<vector<int>> G(N);
    vector<int> indegree(M, 0);

    for (int i = 0; i < M; i++) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        indegree[b]++;
    }

    vector<int> sorted = topological_sort(G, indegree, N);

    if (!accumulate(indegree.begin(), indegree.end(), 0)) {
        for (int i = 0; i < sorted.size(); i++) {
            cout << sorted[i] + 1 << " ";
        }
        cout << endl;
    }
    else {
        cout << -1 << endl;
    }
}