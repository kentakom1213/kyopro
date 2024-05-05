//         D - 2-variable Function         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc246/tasks/abc246_d
// ----------------------------------------

/*
- a -> pow(n, 1/3) まで全探索
- b -> 二分探索
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

ll calc(ll a, ll b) {
    return a*a*a + a*a*b + a*b*b + b*b*b;
}

int main() {
    ll N; cin >> N;

    ll ans = 1LL << 60;

    for (ll a=0; a*a*a <= N; a++) {
        auto isOK = [=](ll b) -> bool {
            return N >= calc(a, b);
        };
        // 2分探索
        ll left=0, right=1010101;
        while (right - left > 1) {
            ll mid = (left + right) / 2;
            if (isOK(mid)) left = mid;
            else right = mid;
        }

        chmin(ans, calc(a, right));
        if (calc(a, left) == N) chmin(ans, calc(a, left));
    }

    cout << ans << endl;
}
