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
constexpr ll INF = 9e18;

template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main(){
    int N, M, P, Q, R;
    cin >> N >> M >> P >> Q >> R;
    // W[i][j] := 女子iから男子jへの重み
    vector<vector<ll>> W(N, vector<ll>(M, 0));
    rep(i, 0, R) {
        int x, y; cin >> x >> y;
        x--; y--;
        ll z; cin >> z;
        W[x][y] = z;
    }

    // S[i][T] := 女子iに対して，男子の集合がTのときの重みの合計
    vector<vector<ll>> S(N, vector<ll>(1 << M, 0));

    rep(i, 0, N) {
        rep(T, 0, 1 << M) {
            rep(j, 0, M) if (T >> j & 1) {
                S[i][T] += W[i][j];
            }
        }
    }

    ll ans = 0;

    rep(T, 0, 1 << M) if (__builtin_popcount(T) == Q) {
        vector<ll> tmp(N);
        // 値のコピー
        rep(j, 0, N) {
            tmp[j] = S[j][T];
        }
        sort(tmp.begin(), tmp.end(), greater<ll>());

        // 大きい順にP個取る
        ll score = 0;
        rep(j, 0, P) {
            score += tmp[j];
        }

        chmax(
            ans,
            score
        );
    }

    cout << ans << endl;

    return 0;
}
