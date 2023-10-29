//               D - 多重ループ
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc021/tasks/abc021_d

// 部分点
// 99 / 100

// AC  × 25
// TLE × 1
// MLE × 1
// RE  × 6
// ----------------------------------------

// 重複組合せの数を列挙する
// 列挙する場合: O(n^k)

// DPテーブル
// dp[i][j] := (i+1)個の要素からj個の数を重複組合せで選択する場合の数

// dp[0][0] = 1
// dp[i+1][j+1] = dp[i][j+1] + dp[i+1][j]

// DPの O(nk)
// n, k <= 10^5 だから最大になると間に合わないかも

#include <bits/stdc++.h>
using namespace std;

const int MOD = 1000000007;

int main() {
    int n, k; cin >> n >> k;
    
    vector<vector<int>> dp(n, vector<int>(k+1, 0));
    dp[0][0] = 1;

    for (int i = 0; i < n; i++) {
        for (int j = 0; j <= k; j++) {
            if (i > 0) dp[i][j] = (dp[i][j] + dp[i-1][j]) % MOD;
            if (j > 0) dp[i][j] = (dp[i][j] + dp[i][j-1]) % MOD;

        }
    }

    // print_array(dp);
    cout << dp[n-1][k] << endl;
}
