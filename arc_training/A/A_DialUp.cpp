//              A - Dial Up
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc125/tasks/arc125_a

// arc...
// ----------------------------------------

// S <= T でない場合は -1
// それ以外は必ず一致させられる

// 最も近い位置にある0/1を記憶しておく？

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N, M; cin >> N >> M;
    vector<int> S(N), T(M);
    rep(i, N) cin >> S[i];
    rep(i, M) cin >> T[i];

    int top = S[0];
    int l=-1, r=-1;  // 左/右シフトして値を変えるときの最小操作回数
    rep(i, N) {
        if (S[i] != top) {
            if (l == -1) l = i;
            r = i;
        }
    }
    r = N - r;

    int ans = 0;
    int now = S[0];
    for (int t : T) {
        if (t != now)  {
            if (l == -1) {
                cout << -1 << endl;
                return 0;
            }
            if (l != 0) {
                ans += min(l, r);
                l = 0;
            } else {
                ans++;
            }
            now ^= 1;
        }
        ans++;
    }
    cout << ans << endl;
}
