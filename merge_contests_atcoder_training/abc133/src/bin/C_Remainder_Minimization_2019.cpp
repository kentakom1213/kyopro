//     C - Remainder Minimization 2019     
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc133/tasks/abc133_c
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
    ll l, r; cin >> l >> r;
    ll ans = 100000;
    for (ll i=l; i <= r; i++) {
        for (ll j=i+1; j <= r; j++) {
            chmin(ans, i*j%2019);
            if (ans == 0) {
                cout << 0 << endl;
                return 0;
            }
        }
    }
    cout << ans << endl;
}