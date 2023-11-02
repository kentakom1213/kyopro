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

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main() {
    ll N, M; cin >> N >> M;
    map<ll, ll> edge;
    rep(i, 0, M) {
        ll a, b; cin >> a >> b;
        ll e = 0;
        rep(j, 0, b) {
            ll c; cin >> c;
            e += 1ll << (c-1);
        }
        edge[e] = a;
    }

    // グラフ上の最短経路問題として解く
    vector<ll> dp(1ll << N, INF);
    dp[0] = 0;
    rep(i, 0, 1ll << N) {
        for (auto [e, p] : edge) {
            ll nxt = i | e;  // 今持っている鍵 | 手に入れる鍵
            chmin(dp[nxt], dp[i] + p);
        }
        print_vector(dp);
    }
    ll ans = dp[(1ll << N) - 1];
    cout << (ans == INF ? -1 : ans) << endl;
}
