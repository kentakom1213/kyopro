//            013 - Passing（★5）            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_m
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// Dijkstra
using P = pair<ll, ll>;
constexpr ll INF = 1001001001001001001;

vector<ll> dijkstra(int start, const vector<vector<P>> &G) {
    int N = G.size();

    // 距離の配列
    vector<ll> dist(N, INF);
    dist[start] = 0;

    // ヒープ
    priority_queue<P, vector<P>, greater<P>> pq;
    pq.push(P(0, start));

    while (!pq.empty()) {
        auto [cost, cur] = pq.top(); pq.pop();
        if (cost > dist[cur]) continue;  // 最短でない場合は無視

        for (auto [nxt, cost_new] : G[cur]) {
            // 緩和処理
            if (dist[cur] + cost_new < dist[nxt]) {
                dist[nxt] = dist[cur] + cost_new;
                pq.push( P(dist[nxt], nxt) );
            }
        }
    }
    return dist;
}

int main() {
    int N, M; cin >> N >> M;
    vector<vector<pll>> G(N);
    for (int i=0; i<M; i++) {
        ll a, b, c; cin >> a >> b >> c;
        a--, b--;
        G[a].emplace_back(b, c);
        G[b].emplace_back(a, c);
    }

    // 頂点1からのダイクストラ
    vector<ll> dist_from_1 = dijkstra(0, G);

    // 頂点Nからのダイクストラ
    vector<ll> dist_from_N = dijkstra(N-1, G);

    for (int i = 0; i < N; i++) {
        ll d = dist_from_1[i] + dist_from_N[i];
        cout << d << endl;
    }
}


