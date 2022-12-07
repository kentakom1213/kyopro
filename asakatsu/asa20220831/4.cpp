// https://atcoder.jp/contests/abc170/tasks/abc170_d

/*
 * ## 方針
 * 割り切れる個数をカウント：エラトステネスの篩
 * 包除原理
 */

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

int main() {
    int N; cin >> N;
    vector<ll> A(N), sieve(1010101);
    rep(i, 0, N) cin >> A[i];

    map<ll, ll> counter;
    for (auto a : A) counter[a]++;

    for (auto [a, v] : counter) {
        for (ll i=2; a*i < 1010101; i++) {
            sieve[a*i]++;
        }
    }

    ll ans = 0;
    for (auto a : A) {
        ans += (sieve[a] == 0 && counter[a] == 1);
    }

    cout << ans << endl;
}