// https://atcoder.jp/contests/agc033/tasks/agc033_a

// [Rustで競技プログラミングの入力をスッキリ記述するマクロ](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8)

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    
    int h; cin >> h;
    int w; cin >> w;
    vector<string> field(h);
    for (int i=0; i<h; i++) cin >> field[i];

    vector<vector<ll>> dist(h, vector<ll>(w, -1));

    // BFS
    deque<pll> deq;
    for (int i=0; i<h; i++) {
        for (int j=0; j<w; j++) {
            if (field[i][j] == '#') {
                dist[i][j] = 0;
                deq.emplace_back(i, j);
            }
        }
    }

    const vector<pll> MOVE = {{0, 1}, {1, 0}, {-1, 0}, {0, -1}};

    while (!deq.empty()) {
        auto [cr, cc] = deq.front();
        deq.pop_front();

        for (auto [dr, dc] : MOVE) {
            ll nr = cr + dr;
            ll nc = cc + dc;

            if (0 <= nr && nr < h
             && 0 <= nc && nc < w
             && dist[nr][nc] == -1) {
                dist[nr][nc] = dist[cr][cc] + 1;
                deq.emplace_back(nr, nc);
            }
        }
    }

    ll ans = 0;
    for (int i=0; i<h; i++) {
        for (int j=0; j<w; j++) {
            chmax(ans, dist[i][j]);
        }
    }

    cout << ans << endl;
}
