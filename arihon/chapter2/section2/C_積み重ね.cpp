//                 C - 積み重ね                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc006/tasks/arc006_3
// ----------------------------------------

/*
- LIS
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
const int INF = 1 << 28;

int main() {
    int N; cin >> N;
    vector<int> dp(N+10, INF);
    
    rep(i, N) {
        int b; cin >> b;
        auto ind = lower_bound(ALL(dp), b);
        *ind = b;
    }

    rep(i, N+1) if (dp[i]==INF) {
        cout << i << endl;
        return 0;
    }
}
