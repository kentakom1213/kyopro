//         B - Counting of Trees
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/nikkei2019-2-qual/tasks/nikkei2019_2_qual_b
// ----------------------------------------

// 木の数え上げ
// 高さ順にソート
// 貪欲に一本ずつ辺を張っていく

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = begin; i < (int)end; i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<long long>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 998244353;

ll pow_mod(ll a, ll n, ll mod) {
    ll res = 1;
    while (n > 0) {
        if (n & 1) res = res * a % mod;
        a = a * a % mod;
        n >>= 1;
    }
    return res;
}

int main() {
    ll N; cin >> N;
    vector<ll> D(N);
    rep(i, N) cin >> D[i];

    // あらかじめカウントしておく
    map<ll, ll> cnt_V;
    rep(i, N) cnt_V[D[i]]++;
    ll max_depth = cnt_V.rbegin()->first;

    if (D[0] != 0 || cnt_V[0] != 1) {
        cout << 0 << endl;
        return 0;
    }

    ll ans = 1;
    range(i, 1, max_depth+1) {
        if (cnt_V[i] == 0) {
            cout << 0 << endl;
            return 0;
        }

        ans *= pow_mod(cnt_V[i-1], cnt_V[i], MOD);
        ans %= MOD;
    }

    cout << ans << endl;
    return 0;
}
