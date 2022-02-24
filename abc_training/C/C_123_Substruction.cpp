//              C - 123引き算
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc011/tasks/abc011_3
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
constexpr int INF = 1<<30;

int main() {
    int N, a, b, c; cin >> N >> a >> b >> c;
    int dp[1000];
    rep(i, 1000) dp[i] = INF;

    dp[0] = 0;
    rep(i, N) {
        if (dp[i]!=-1) {
            if (i==a||i==b||i==c) continue;
            chmin(dp[i+1], dp[i]+1);
            chmin(dp[i+2], dp[i]+1);
            chmin(dp[i+3], dp[i]+1);
        }
    }

    if (dp[N]<=100 && N!=a && N!=b && N!=c) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
}
