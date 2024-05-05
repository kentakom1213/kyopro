//                E - Train                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc192/tasks/abc192_e

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
const ll INF = 1LL << 50;
typedef pair<ll, int> P;

const int MAX_N = 101010;
int N, M, X, Y;
ll dist[MAX_N];

// x以上で最小のkの倍数を返す
ll min_mul(ll x, ll k) {
    return (x + k - 1) / k * k;
}

int main() {
    cin >> N >> M >> X >> Y;
    X--, Y--;
    vector<vector<array<ll, 3>> > G(N);
    rep(i, M) {
        ll a, b, t, k; cin >> a >> b >> t >> k;
        a--, b--;
        G[a].push_back({b, t, k});
        G[b].push_back({a, t, k});
    }

    // Dijkstra
    priority_queue<P, vector<P>, greater<P> > pq;
    pq.push({0, X});

    rep(i, N) {
        if (i == X) dist[i] = 0;
        else dist[i] = INF;
    }

    while (!pq.empty()) {
        auto [w, cur] = pq.top(); pq.pop();
        if (dist[cur] < w) continue;  // 最短でない場合は無視

        for (auto [nxt, t, k] : G[cur]) {
            ll new_time = min_mul(dist[cur], k) + t;
            if (new_time < dist[nxt]) {
                dist[nxt] = new_time;
                pq.push({dist[nxt], nxt});
            }
        }
    }

    cout << (dist[Y]==INF ? -1 : dist[Y]) << endl;
}
