//             E - Sequence Sum            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc179/tasks/abc179_e
// ----------------------------------------

/*
## 方針
- 鳩の巣原理から、ループがあることがわかる
- メモ化しておく
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int MAX_SIZE = 101010;
ll N, X, M;
ll S[MAX_SIZE], loop[MAX_SIZE];

int main() {
    cin >> N >> X >> M;
    rep(i, MAX_SIZE) S[i] = loop[i] = -1;
    S[0] = 0;

    ll loop_begin=-1, loop_end=-1;
    rep(i, MAX_SIZE-1) {
        S[i+1] = S[i] + X;
        if (loop[X] == -1) {
            loop[X] = i;
        } else {
            loop_begin = loop[X];
            loop_end = i;
            break;
        }
        X = X*X % M;
    }

    // ループ前
    ll ans = S[loop_begin];
    N -= loop_begin;

    // ループ中
    ll loop_size = loop_end - loop_begin;
    ans += (S[loop_end] - S[loop_begin]) * (N / loop_size);

    // ループ後
    N %= loop_size;
    ans += S[loop_begin + N] - S[loop_begin];

    cout << ans << endl;
}
