//           E - Putting Candies           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc241/tasks/abc241_e
// ----------------------------------------

/*

## 方針
- ループ検出

*/

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

template<class T>
void print_map(map<T, pll> dict) {
    cerr << "{\n";
    for (auto &[a, b] : dict) {
        cerr << "   {" << a << ", {" << b.first << ": " << b.second << "} },\n";
    }
    cerr << "}" << endl;
}

int main() {
    ll n, k; cin >> n >> k;
    vector<ll> a(n);
    for (int i=0; i<n; i++) cin >> a[i];

    ll now=0, i=0;
    map<ll, pll> mp;
    while (i < k && mp.find(now%n) == mp.end()) {
        mp[now%n] = pll(i, now);
        now += a[now%n];
        i++;
    }

    // if (i >= k) {
    //     cout << now << endl;
    //     return 0;
    // }

    ll loop_len = i - mp[now%n].first;
    ll par_loop = now - mp[now%n].second;
    ll loop_cnt = k / loop_len;
    ll loop_rem = k % loop_len;

    printf("loop_len: %lld\npar_loop: %lld\nloop_cnt: %lld\nloop_rem: %lld\n", loop_len, par_loop, loop_cnt, loop_rem);

    ll ans = par_loop * loop_cnt;

    // ループの残りを加算
    now %= n;
    while (loop_rem--) {
        ans += a[now];
        now = (now + a[now]) % n;
    }

    cout << ans << endl;
}
