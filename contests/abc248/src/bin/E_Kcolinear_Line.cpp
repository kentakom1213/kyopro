//           E - K-colinear Line           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc248/tasks/abc248_e
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

typedef pair<ll, ll> P;
typedef pair<P, P> PP;

// ----- for debug -----
void print_PP(const PP &pp) {
    auto &[p1, p2] = pp;
    auto &[a1, b1] = p1;
    auto &[a2, b2] = p2;
    printf("((%lld, %lld), (%lld, %lld))", a1, b1, a2, b2);
}
// ---------------------

ll N, K;

ll gcd(ll x, ll y) {
    if (y == 0) return x;
    return gcd(y, x%y);
}

// 約分する
void reduce(P &p) {
    auto &[x, y] = p;
    ll m = gcd(x, y);
    x /= m, y /= m;

    if (x*y<0) x=-abs(x), y=abs(y);
}

int main() {
    // nC2 の逆関数テーブル
    map<ll, ll> comb_inv;
    rep(i, 1, 400) {
        comb_inv[i*(i-1)/2] = i;
    }

    cin >> N >> K;
    P points[N];

    for (auto &[x, y] : points) cin >> x >> y;

    if (K == 1) {
        cout << "Infinity" << endl;
        return 0;
    }

    // O(N^2) で直線 l: y=Ax+B を列挙
    map<PP, ll> lines;
    rep(i, 0, N) {
        rep(j, i+1, N) {
            auto [x1, y1] = points[i];
            auto [x2, y2] = points[j];
            P A = {y2-y1, x2-x1}; reduce(A);
            P B = {x2*y1 - x1*y2, x2 - x1}; reduce(B);

            if (A == make_pair(1LL, 0LL)) {
                B = make_pair(x1, 0LL);
            }

            // -- debug --
            // print_PP({points[i], points[j]});
            // cout << " -> ";
            // print_PP({A, B}); cout << "\n";

            lines[{A, B}]++;
        }
    }

    ll ans = 0;
    for (auto [pp, n] : lines) {
        // print_PP(pp); printf(" -> %lld -> %lld\n", n, comb_inv[n]);
        ll p = comb_inv[n];
        if (p >= K) ans++;
    }

    cout << ans << endl;
}
