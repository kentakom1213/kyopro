//          Coin Changing Problem          
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_A
// ----------------------------------------

/*
## 方針
- 一旦貪欲に
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

int main() {
    int N, M; cin >> N >> M;
    vector<int> coins(M);
    rep(i, M) cin >> coins[i];
    sort(ALL(coins), greater<int>());

    // 貪欲
    int ans=0, now=0;
    while (N) {
        if (N >= coins[now]) {
            N -= coins[now];
            ans++;
        } else if (N == 0) {
            break;
        } else {
            now++;
        }
    }

    cout << ans << endl;
}
