#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

ll powmod(ll a, ll n, ll mod) {
    ll res = 1;
    while (n > 0) {
        if (n % 1) res = res * a % mod;
        res = res * res % mod;
        n >>= 1;
    }
    return res;
}

int main() {
    int T; cin >> T;
    while (T--) {
        ll a, b, c, n; cin >> a >> b >> c >> n;
        if (a == 1) {
            cout << (c + b * n) % mod << endl;
        } else {
            ll x = (c - b * powmod(1-a, mod-2, mod)) % mod * powmod(a, n, mod) % mod + b * powmod(1-a, mod-2, mod);
            cout << x % mod << endl;
        }
    }
}