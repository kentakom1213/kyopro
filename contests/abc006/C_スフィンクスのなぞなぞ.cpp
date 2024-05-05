//             C - スフィンクスのなぞなぞ             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc006/tasks/abc006_3
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

ll N, M;

int main() {
    cin >> N >> M;
    ll l = M - 3*N, r = (M - 2*N) / 2;

    for (ll z=l; z<=r; z++) {
        ll x = z + 3*N - M;
        ll y = -2*z + M - 2*N;
        if (x >= 0 && y >= 0 && z >= 0) {
            printf("%lld %lld %lld\n", x, y, z);
            return 0;
        }
    }
    printf("-1 -1 -1\n");
}
