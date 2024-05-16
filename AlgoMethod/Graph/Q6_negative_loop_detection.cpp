#include <bits/stdc++.h>
using namespace std;

constexpr int INF = 1001001001;

int main() {
    int N, M;
    cin >> N >> M;
    vector<array<int, 3> > edges(M);
    for (auto &[a, b, c] : edges) {
        cin >> a >> b >> c;
    }

    // ベルマン・フォード法
    vector<int> dist(N, INF);
    dist[0] = 0;

    for (int i = 0; i < N; i++) {
        for (auto [u, v, w] : edges) {
            // 緩和
            if (dist[v] > dist[u] + w) {
                dist[v] = dist[u] + w;
            }
        }
    }

    // 負閉路検出
    bool has_neg_loop = false;

    for (int i = 0; i < N; i++) {
        for (auto [u, v, w] : edges) {
            // 更に更新できる場合
            has_neg_loop |= dist[v] > dist[u] + w;
        }
    }

    cout << (has_neg_loop ? "Yes" : "No") << endl;
}
