//              D - Knapsack 1
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_d

// AC
// ----------------------------------------

#include <vector>
#include <iostream>
using namespace std;

int main() {
    int N, W; cin >> N >> W;
    vector<int> weights(N), values(N);
    for (int i = 0; i < N; i++) cin >> weights[i] >> values[i];

    vector<vector<long long>> dp(101, vector<long long>(100100, 0));
    // vector<vector<int>> dp(N+1, vector<int>(W+1, 0));
    for (int i = 0; i < N; i++) {
        for (int j = 0; j <= W; j++) {
            // 含める場合
            if (j + weights[i] <= W) {
                dp[i+1][j+weights[i]] = max(
                    dp[i+1][j+weights[i]],
                    dp[i][j] + values[i]
                );
            }
            // 含めない場合
            dp[i+1][j] = max(dp[i+1][j], dp[i][j]);
        }
    }
    cout << dp[N][W] << endl;
}