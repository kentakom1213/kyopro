//                 A - 経過日数                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc023/tasks/arc023_1
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int y, m, d; cin >> y >> m >> d;
    if (m <= 2) {
        y -= 1;
        m += 12;
    }

    auto calc = [](int y, int m, int d) {
        return 365*y + y/4 - y/100 + y/400 + 306*(m+1)/10 + d - 429;
    };

    cout << (
        calc(2014, 5, 17) - calc(y, m, d)
    ) << endl;
}
