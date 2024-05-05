//             E - Knapsack 2
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_e

// AC
// ----------------------------------------

// Wの値が非常に大きいため、Wをdpテーブルのキーにすることは不可能
// valueを満たすweightの最小値を求めるようにdpテーブルを設定

// DPテーブル
// dp[i][j] := i番目までの荷物を使って価値jを表現するときのweightの最小値

// 初期条件
// dp[0][0] = 0
// dp[_][_] = INF

// 漸化式
// ( j + value(i) <= 10^3 * 100 )
// dp[i+1][j+value(i)] = chmin( dp[i][j] + weights[i] )
// dp[i+1][j] = dp[i][j]


#include <iostream>
#include <vector>
using namespace std;

const int INF = 1000000001;  // W <= 10^9

int main() {
    int N, W; cin >> N >> W;
    vector<int> weights(N), values(N);
    for (int i = 0; i < N; i++) cin >> weights[i] >> values[i];

    // dp
    int max_value = 101000;
    vector<vector<int>> dp(N+1, vector<int>(max_value, INF));
    dp[0][0] = 0;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < max_value; j++) {
            if (j + values[i] <= max_value) {
                dp[i+1][j+values[i]] = min(
                    dp[i+1][j+values[i]],
                    dp[i][j] + weights[i]
                );
            }
            dp[i+1][j] = min(
                dp[i+1][j],
                dp[i][j]
            );
        }
    }

    int res = 0;
    for (int i = 0; i < max_value; i++) {
        if (dp[N][i] <= W) {
            res = i;
        }
    }
    cout << res << endl;
}