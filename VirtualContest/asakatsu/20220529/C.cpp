
#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (ll i = (ll)(a); i < (ll)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

ll gcd(ll a, ll b) {
    if (b == 0) return a;
    return gcd(b, a%b);
}

int main() {
    ll l, r; cin >> l >> r;
    
    // 左から狭めていく
    ll ans_l = 0;
    rep(i, l, r) {
        if (gcd(i, r) == 1) {
            ans_l = r - i;
            break;
        }
    }

    // 右から狭めていく
    ll ans_r = 0;
    for (ll i=r; i>l; i--) {
        if (gcd(l, r) == 1) {
            ans_r = i - l;
            break;
        }
    }

    cout << max(ans_l, ans_r) << endl;
}