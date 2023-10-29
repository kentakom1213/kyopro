//               H - Grid 1
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_h

// AC
// ----------------------------------------

// 漸化式
// dp[i][j] := マス(i, j)に到達するまでの最短経路の組み合わせ(% MOD)

// dp[0][0] = 0
// dp[i][j] = | dp[i][j-1] + 1  (i = 0)
//            | dp[i-1][j] + 1  (j = 0)
//            | (dp[i-1][j] + dp[i][j-1]) % MOD  (i, j != 0)


#include <bits/stdc++.h>
using namespace std;

const int MOD = 1000000007;

int main() {
    int H, W; cin >> H >> W;
    vector<string> field(H);
    for (int i = 0; i < H; i++) cin >> field[i];

    vector<vector<int>> dp(H, vector<int>(W, 0));
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            if (field[i][j] == '.') {
                if (i == 0 && j == 0) dp[i][j] = 1;
                else if (i == 0) dp[i][j] = dp[i][j-1];
                else if (j == 0) dp[i][j] = dp[i-1][j];
                else {
                    dp[i][j] = (dp[i][j-1] + dp[i-1][j]) % MOD;
                }
            }
        }
    }
    cout << dp[H-1][W-1] << endl;
}