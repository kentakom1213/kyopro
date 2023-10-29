//            E - Get Everything           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc142/tasks/abc142_e
// ----------------------------------------

// https://atcoder.jp/contests/abc142/tasks/abc142_e

#include <bits/stdc++.h>
using namespace std;
using ll = long long;
#define rep(i, a, b) for (ll i = (ll)(a); i < (ll)(b); i++)
#define ALL(A) A.begin(), A.end()
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr ll MOD = 1000000007;
constexpr ll mod = 998244353;
constexpr ll INF = 1 << 30;
using pll = pair<ll, ll>;

int main() {
    ll N, M; cin >> N >> M;
    vector<pll> edge(M);
    rep(i, 0, M) {
        ll a, b; cin >> a >> b;
        ll e = 0;
        rep(j, 0, b) {
            ll c; cin >> c;
            e += 1ll << (c-1);
        }
        edge[i] = {e, a};
    }

    // グラフ上の最短経路問題として解く
    vector<vector<ll>> dp(M+1, vector(1<<N, INF));

    dp[0][0] = 0;
    rep(i, 0, M) {
        auto [e, a] = edge[i];
        rep(j, 0, 1ll << N) {
            chmin(dp[i+1][j], dp[i][j]);
            ll nxt = j | e;
            chmin(dp[i+1][nxt], min(dp[i][j]+a, dp[i][nxt]));
        }
    }

    ll ans = dp[M][(1ll << N) - 1];
    cout << (ans == INF ? -1 : ans) << endl;
}
