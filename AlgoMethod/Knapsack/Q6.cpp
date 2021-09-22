//          問題 6：K 個以内部分和問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/312

// AC
// ----------------------------------------

// 漸化式について
// dp[i+1][j] := iまでの数の組み合わせでjを表すために必要な最小個数
// dp[i+1][j] = min| dp[i][j-Ai] + 1
//                 | dp[i][j]
// dp[0][0] = 0

#include <bits/stdc++.h>
using namespace std;

const int INF = 1 << 29;

int main() {
    int N, M, K; cin >> N >> M >> K;
    vector<vector<int>> dp(N+1, vector<int>(M+1, INF));
    dp[0][0] = 0;

    for (int i = 0; i < N; i++) {
        int Ai; cin >> Ai;
        for (int j = 0; j <= M; j++) {

            if (j >= Ai) {
                dp[i+1][j] = min(
                    dp[i][j-Ai] + 1,
                    dp[i][j]
                );
            }
            // 真下に下ろす
            dp[i+1][j] = min(dp[i][j], dp[i+1][j]);
        }
    }
    cout << (dp[N][M] <= K ? "Yes" : "No") << endl;
}