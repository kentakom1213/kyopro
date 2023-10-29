//           D - Together Square           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc254/tasks/abc254_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
constexpr ll INF = numeric_limits<long long>::max();

constexpr ll MAX_N = 202020;
constexpr ll MAX_N_SQ = 40404040404;

template<class T>
T ceil_div(T a, T b) {
    return (a + b - 1) / b;
}

map<ll, ll> factoring(ll n) {
    map<ll, ll> mp;
    for (ll i = 2; i*i <= n; i++) {
        while (n % i == 0) {
            mp[i]++;
            n /= i;
        }
    }
    if (n != 1) mp[n]++;
    return mp;
}

int main() {
    ll N; cin >> N;

    ll ans = 0;
    for (ll n = 1; n <= N; n++) {
        auto mp = factoring(n);

        ll tmp = 1;
        for (auto [k, v] : mp) {
            tmp *= ceil_div(v+1, 2LL);
        }
        ans += tmp * 2 - 1;
    }

    cout << ans << endl;
}
