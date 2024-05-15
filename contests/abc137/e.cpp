#include <bits/stdc++.h>
// #include <atcoder/all>
using namespace std;
// using namespace atcoder;
#define rep(i, a, n) for(int i = a; i < n; i++)
#define rrep(i, a, n) for(int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

// using mint = modint1000000007;
// using mint = modint998244353;
constexpr int IINF = 1001001001;
constexpr ll INF = 1001001001001001001;

using pii = pair<int, int>;
using pil = pair<int, ll>;
template<class t,class u> void chmax(t&a,u b){if(a<b)a=b;}
template<class t,class u> void chmin(t&a,u b){if(b<a)a=b;}

int N, M, P;

// グラフ
vector<array<int, 3> > ABC;
map<pii, ll> edge;

// 頂点sから頂点tへのコストの最小値を求める．
// コストを限りなく小さくできるとき，INFを返す
ll bellman_ford(int s, int t) {
    // 頂点iまでのコストの最小値
    vector<ll> dist(N, INF);
    dist[s] = 0;
    
    rep(_, 0, N) {
        for (auto [u, v, w] : ABC) {
            // 緩和処理
            if (dist[u] != INF && dist[v] > dist[u] + w) {
                dist[v] = dist[u] + w;
            }
        }
    }

    // 負閉路の検出
    vector<bool> neg(N, false);

    rep(i, 0, N) {
        for (auto [u, v, w] : ABC) {
            if (dist[v] != INF && dist[v] > dist[u] + w) {
                dist[v] = dist[u] + w;
                neg[v] = true;
            }
            if (neg[u]) {
                neg[v] = true;
            }
        }
    }

    if (neg[t]) {
        return INF;
    } else {
        return dist[t];
    }
}

int main(){
    cin >> N >> M >> P;

    ABC.assign(M, {0, 0, 0});

    rep(i, 0, M) {
        int a, b, c;
        cin >> a >> b >> c;
        
        ABC[i] = {
            a - 1,
            b - 1,
            P - c,
        };
    }

    // ベルマンフォード法
    auto ans = bellman_ford(0, N - 1);

    if (ans == INF) {
        cout << -1 << endl;
    } else {
        cout << max(-ans, (ll)0) << endl;
    }
    
    return 0;
}
