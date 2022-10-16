//     037 - Don't Leave the Spice（★5）     
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ak
// ----------------------------------------

/*

## 愚直な解法
dpをする
dp[i][j] := i個めまでの料理で合計j[mg]の香辛料を使っているときの価値の最大値

## 方針
- セグ木でdpを高速化

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
constexpr ll INF = 1001001001001001001;

template <typename T>
void print_array(vector<vector<T>>& array) {
    int H = array.size();

    cerr << "{" << endl;
    for (int i = 0; i < H; i++) {
        int W = array.at(i).size();
        cerr << "  {";
        for (int j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
}

int main() {
    ll w, n; cin >> w >> n;
    
    vector<vector<ll>> dp(n+1, vector<ll>(w+1, -1));
    dp[0][0] = 0;

    for (int i = 1; i <= n; i++) {
        ll l, r, v; cin >> l >> r >> v;
        for (int j = 0; j <= w; j++) {
            chmax(dp[i][j], dp[i-1][j]);
            for (int k = l; k <= r; k++) {
                if (0 <= j-k && j-k <= w && dp[i-1][j-k] >= 0) {
                    chmax(dp[i][j], dp[i-1][j-k] + v);
                }
            }
        }
    }

    print_array(dp);

    cout << dp[n][w] << endl;
}
