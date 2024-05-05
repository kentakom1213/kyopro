//             D - Kth Excluded            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc205/tasks/abc205_d
// ----------------------------------------

// 苦戦した

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

ll N, Q;

int main() {
    cin >> N >> Q;
    vector<ll> A(N+1), span(N+1);
    rep(i, 0, N) cin >> A[i];
    sort(ALL(A));

    // 差分の累積和を取る
    rep(i, 1, N+1) {
        span[i] = span[i-1] + A[i] - A[i-1] - 1;
    }

    while (Q--) {
        ll k;
        cin >> k;
        int idx = lower_bound(ALL(span), k) - span.begin();
        ll ans = A[idx-1] + k - span[idx-1];
        cout << ans << endl;
    }
}
