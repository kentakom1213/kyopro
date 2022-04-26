//             A - みんなでワイワイみかん             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc056/tasks/arc056_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, A, B) for (int i = (int)(A); i < (int)(B); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &A, const T B) { if (A < B) { A = B; return true; } return false; }
template <typename T> inline bool chmin(T &A, const T B) { if (A > B) { A = B; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

ll A, B, K, L;

int main() {
    cin >> A >> B >> K >> L;
    ll smin=K/L, smax=(K+L-1)/L;

    ll ans = (ll)(1e20);
    for (ll x=smin; x<=smax; x++) {
        chmin(ans, B*x + A * max(K-x*L, 0LL));
    }

    cout << ans << endl;
}
