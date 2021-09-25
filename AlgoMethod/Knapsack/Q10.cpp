// 問題 10：レーベンシュタイン距離 (diff コマンド)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/315

// 要復習

// AC (解説)
// ----------------------------------------

#include <iostream>
#include <vector>
using namespace std;

const int INF = 1 << 29;

int main() {
    string S, T; cin >> S >> T;
    int ls = S.size(), lt = T.size();
    vector<vector<int>> dp(ls+1, vector<int>(lt+1, INF));
    dp[0][0] = 0;

    for (int i = -1; i < ls; i++) {
        for (int j = -1; j < lt; j++) {
            if (i == -1 && j == -1) continue;
            if (i >= 0 && j >= 0) {
                if (S[i] == T[j]) dp[i+1][j+1] = min(dp[i+1][j+1], dp[i][j]);
                else dp[i+1][j+1] = min(dp[i+1][j+1], dp[i][j] + 1);
            }
            if (i >= 0) dp[i+1][j+1] = min(dp[i+1][j+1], dp[i][j+1] + 1);
            if (j >= 0) dp[i+1][j+1] = min(dp[i+1][j+1], dp[i+1][j] + 1);
        }
    }
    // print_array(dp);
    cout << dp[ls][lt] << endl;
}