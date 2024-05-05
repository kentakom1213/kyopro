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
constexpr ll INF = 1001001001001001001;

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
    vector<ll> dist_from_1(N, INF);
    dist_from_1[0] = 0;

    priority_queue<pll, vector<pll>, greater<pll>> pq1;
    pq1.push(pll(0, 0));
    while (!pq1.empty()) {
        auto [cost, u] = pq1.top();
        pq1.pop();

        if (cost > dist_from_1[u]) continue;

        for (auto [v, cost_new] : G[u]) {
            if (dist_from_1[u] + cost_new > dist_from_1[v]) continue;
            dist_from_1[v] = dist_from_1[u] + cost_new;
            pq1.push(pll(dist_from_1[v], v));
        }
    }

    // 頂点Nからのダイクストラ
    vector<ll> dist_from_N(N, INF);
    dist_from_N[N-1] = 0;  // 始点はN

    priority_queue<pll, vector<pll>, greater<pll>> pqN;
    pqN.push(pll(0, N-1));
    while (!pqN.empty()) {
        auto [cost, u] = pqN.top();
        pqN.pop();

        if (cost > dist_from_N[u]) continue;

        for (auto [v, cost_new] : G[u]) {
            if (dist_from_N[u] + cost_new > dist_from_N[v]) continue;  // すでに最短経路の場合
            dist_from_N[v] = dist_from_N[u] + cost_new;
            pqN.push(pll(dist_from_N[v], v));
        }
    }

    for (int i = 0; i < N; i++) {
        ll d = dist_from_1[i] + dist_from_N[i];
        cout << d << endl;
    }
}


