//            C - Base -2 Number           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc105/tasks/abc105_c
// ----------------------------------------

/*
## 方針
- 半分全列挙でゴリ押し
*/

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
    ll N; cin >> N;

    if (N == 0) {
        cout << 0 << endl;
        return 0;
    }

    map<ll, ll> over20;
    for (ll i=0; i < (1LL << 20); i++) {
        ll n = 0;
        for (ll j=0; j < 20; j++) {
            if ((i>>j) & 1) {
                if (j & 1) n -= (1LL << j) << 20;
                else n += (1LL << j) << 20;
            }
        }
        over20[n] = i << 20;
    }

    ll ans = 0;
    for (ll i=0; i < (1LL << 20); i++) {
        ll m = 0;
        for (ll j=0; j < 20; j++) {
            if ((i>>j) & 1) {
                if (j & 1) m -= 1LL << j;
                else m += 1LL << j;
            }
        }

        // 2分探索
        ll rem = N - m;
        if (over20.find(rem) != over20.end()) {
            ans = over20[rem] + i;
            break;
        }
    }

    stack<bool> bin;
    while (ans) {
        bin.push( ans & 1 );
        ans >>= 1;
    }

    while (!bin.empty()) {
        cout << bin.top(); bin.pop();
    }
    cout << endl;
}
