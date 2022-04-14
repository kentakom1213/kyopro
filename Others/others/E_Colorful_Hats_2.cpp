//           E - Colorful Hats 2           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_e
// ----------------------------------------

/*
cnt[i] := i以前に使われているA[i]の個数

- 非常に難しかった
*/

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

int N, A[101010];
int cnt[101010];

int main() {
    cin >> N;
    rep(i, N) {
        cin >> A[i];
        A[i]++;
    }

    ll ans = 1;
    cnt[0] = 3;
    rep(i, N) {
        if (0 < cnt[A[i] - 1]) {
            ans = ans * cnt[A[i] - 1] % MOD;
            cnt[A[i] - 1]--;
            cnt[A[i]]++;
        } else {
            cout << 0 << endl;
            return 0;
        }
    }
    cout << ans << endl;
}
