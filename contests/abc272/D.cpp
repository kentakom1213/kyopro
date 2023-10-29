
#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

bool can_reach(ll x, ll y, ll  N) {
    return 0 <= x < N && 0 <= y < N;
}

int main() {
    ll N, M; cin >> N >> M;

    // 平方数の列挙
    vector<pll> moves;
    for (ll i = 0; i*i <= M; i++) {
        for (ll j = 0; i*i + j*j <= M; j++) {
            if (i*i + j*j == M) {
                moves.emplace_back(i, j);
            }
        }
    }

    vector<pll> SIGN = {{1, 1}, {1, -1}, {-1, 1}, {-1, -1}};

    // BFS
    vector field(N, vector<ll>(N, (ll)1e18));
    field[0][0] = 0;

    deque<pll> que;
    que.emplace_back(0, 0);

    while (!que.empty()) {
        auto [cr, cc] = que.front();
        que.pop_front();

        for (auto [dr, dc] : moves) {
            for (auto [sr, sc] : SIGN) {
                ll nr = cr + dr * sr;
                ll nc = cc + dc * sc;
                if (can_reach(nr, nc, N)) {
                    bool isMin = chmin(field[nr][nc], field[cr][cc] + 1);
                    if (isMin) {
                        que.emplace_back(nr, nc);
                    }
                }
            }
        }
    }

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            ll out = field[i][j];
            cout << (out >= (ll)1e18 ? -1 : out) << " ";
        }
        cout << "\n";
    }
}