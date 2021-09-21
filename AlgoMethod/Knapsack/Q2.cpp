//          問題 2：ナップサック問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/308

// 配列外参照には気をつけましょう

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, W; cin >> N >> W;

    vector<int> weights(N), values(N);
    for (int i = 0; i < N; i++) cin >> weights[i] >> values[i];

    vector<vector<int>> dp(N+1, vector<int>(W+1, 0));
    for (int i = 0; i < N; i++) {
        int w = weights[i], v = values[i];

        for (int sum_w = 0; sum_w <= W; sum_w++) {
            // 追加する場合
            if (sum_w + w <= W) {
                dp[i+1][sum_w + w] = max(
                    dp[i][sum_w + w],
                    dp[i][sum_w] + v
                );
            }
            // 追加しない場合
            dp[i+1][sum_w] = max(
                dp[i][sum_w],
                dp[i+1][sum_w]
            );
        }
    }
    cout << dp[N][W] << endl;
}