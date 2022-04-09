//                 A - おつり                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int coins[] = {500, 100, 50, 10, 5, 1};

int main() {
    int pay; cin >> pay;
    int change = 1000 - pay;

    // 貪欲法
    int ans = 0, now = 0;
    while (change) {
        if (change >= coins[now]) {
            change -= coins[now];
            ans++;
        } else if (change == 0) {
            break;
        } else {
            now++;
        }
    }

    cout << ans << endl;
}
