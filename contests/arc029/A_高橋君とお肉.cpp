//                A - 高橋君とお肉               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc029/tasks/arc029_1
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int T[5];

int main() {
    int N; cin >> N;
    rep(i, N) cin >> T[i];

    // bit全探索
    int ans = 1 << 28;
    rep(i, 1<<N) {
        int a=0, b=0;
        rep(j, N) {
            if ((i>>j)&1) {
                a += T[j];
            } else {
                b += T[j];
            }
        }
        chmin(ans, max(a, b));
    }

    cout << ans << endl;
}
