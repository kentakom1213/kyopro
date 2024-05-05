//                B - □□□□□                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc040/tasks/abc040_b
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

int main() {
    int n; cin >> n;

    int ans = 1<<30;
    for (int i=1; i*i<=n; i++) {
        for (int j=i; i*j<=n; j++) {
            chmin(ans, j-i + n-i*j);
        }
    }
    cout << ans << endl;
}
