//                 B - 方程式                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc034/tasks/arc034_2

// AC
// ----------------------------------------

/*
- 桁ごとに考える
*/

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

ll N;

ll f(ll x) {
    ll res = 0;
    while (x) {
        res += x % 10;
        x /= 10;
    }
    return res;
}

int main() {
    cin >> N;

    set<ll> ans;
    for (ll i=1; i<19; i++) { // 桁ごとに考える
        for (ll x=N-9*i; x<N; x++) {
            if (x + f(x) == N) ans.insert(x);
        }
    }

    cout << ans.size() << endl;
    for (auto v : ans) cout << v << endl;
}
