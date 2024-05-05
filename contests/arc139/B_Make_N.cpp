//                B - Make N               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc139/tasks/arc139_b
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

void test() {
    ll n, a, b, x, y, z;
    cin >> n >> a >> b >> x >> y >> z;
    if (a < b) swap(a, b), swap(y, z);

    if (n/a < a-1) {
        ll ans = 1LL<<50;
        for (ll k = 0; k*a <= n; k++) {
            ll 
        }
    } else {

    }
}

int main() {
    int T; cin >> T;
    while (T--) test();
}
