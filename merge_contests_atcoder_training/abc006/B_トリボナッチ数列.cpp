//               B - トリボナッチ数列              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc006/tasks/abc006_2
// ----------------------------------------

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
constexpr int MOD = 10007;

int main() {
    int n; cin >> n;
    int dp[2000000];
    dp[1] = dp[2] = 0;
    dp[3] = 1;

    range(i, 1, n) {
        dp[i+3] = (dp[i] + dp[i+1] + dp[i+2]) % MOD;
    }

    cout << dp[n] << endl;
}
