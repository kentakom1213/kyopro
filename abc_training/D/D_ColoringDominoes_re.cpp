//         D - Coloring Dominoes
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc081/tasks/arc081_b
// ----------------------------------------

// 左側のパターンで分類
// 1.
//   AX
//   AX   -> 2通り
// 2.
//   AAX
//   BBX  -> 1通り
// 3.
//   AXX
//   AYY  -> 2通り
// 4.
//   AAXX
//   BBYY -> 3通り

#include <bits/stdc++.h>
using namespace std;
#define rep(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;

int main() {
    int N; cin >> N;
    string s1, s2; cin >> s1 >> s2;
    
    ll ans = (s1[0]==s2[0] ? 3 : 6);
    rep(i, 1, N) {
        if (s1[i] == s1[i-1]) continue;
        if (s1[i] == s2[i]) {
            if (s1[i-1] == s2[i-1]) ans *= 2;
            else                    ans *= 1;
        } else {
            if (s1[i-1] == s2[i-1]) ans *= 2;
            else                    ans *= 3;
        }
        ans %= MOD;
    }
    cout << ans << endl;
}
