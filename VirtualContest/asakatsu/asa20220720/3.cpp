// https://atcoder.jp/contests/abc175/tasks/abc175_c

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

#define abs(x) ((x)>0 ? x : -(x))

ll solve(ll x, ll k, ll d) {
    x = abs(x);
    k -= x / d;
    if (k <= 0) return x;
    if (k & 1) {
        return abs(x - d);
    } else {
        return x;
    }
}

int main() {
    ll X, K, D; cin >> X >> K >> D;
    cout << solve(X, K, D) << endl;
}