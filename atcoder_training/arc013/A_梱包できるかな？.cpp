//               A - 梱包できるかな？              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc013/tasks/arc013_1
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

int main() {
    ll n, m, l, p, q, r; cin >> n >> m >> l >> p >> q >> r;
    ll ans = 0;

    chmax(ans, (n/p) * (m/q) * (l/r));
    chmax(ans, (n/p) * (m/r) * (l/q));
    chmax(ans, (n/q) * (m/p) * (l/r));
    chmax(ans, (n/q) * (m/r) * (l/p));
    chmax(ans, (n/r) * (m/p) * (l/q));
    chmax(ans, (n/r) * (m/q) * (l/p));

    cout << ans << endl;
}