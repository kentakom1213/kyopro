//         問題 4：部分和数え上げ問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/310
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M; cin >> N >> M;
    vector<vector<int>> dp(101, vector<int>(100000, 0));
    dp[0][0] = 1;

    for (int i = 0; i < N; i++) {
        int Ai; cin >> Ai;
        for (int j = 0; j <= M; j++) {
            if (dp[i][j]) {
                if (j + Ai <= M) dp[i+1][j+Ai] = dp[i][j] + dp[i][j+Ai];
                dp[i+1][j] = max(dp[i][j], dp[i+1][j]) % 1000;
            }
        }
    }
    cout << dp[N][M] << endl;
}