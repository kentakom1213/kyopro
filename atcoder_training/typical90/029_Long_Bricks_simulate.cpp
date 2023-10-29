//          029 - Long Bricks（★5）          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ac
// ----------------------------------------

/*

## 方針
- 小課題のACを目指す
- 愚直なシミュレート：O(NW)

*/

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
    int W, N; cin >> W >> N;
    vector<pll> LR(N);
    for (auto &[l, r] : LR) {
        cin >> l >> r;
        l--;
    }

    vector<ll> range(W, 0), renga(N, 0);

    for (int i = 0; i < N; i++) {
        auto [l, r] = LR[i];

        ll maxi = 0;
        for (int i = l; i < r; i++) {
            chmax(maxi, range[i]);
        }
        
        for (int i = l; i < r; i++) {
            range[i] = maxi + 1;
        }

        renga[i] = maxi + 1;
    }

    for (auto v : renga) cout << v << endl;
}
