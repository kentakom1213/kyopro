//         問題 5：最小個数部分和問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/311

// AC
// ----------------------------------------

// AC
#include <bits/stdc++.h>
using namespace std;

const int INF = 1 << 29;

int main() {
    int N, M; cin >> N >> M;
    vector<vector<int>> dp(N+1, vector<int>(M+1, INF));
    dp[0][0] = 0;

    for (int i = 0; i < N; i++) {
        int Ai; cin >> Ai;
        for (int j = 0; j <= M; j++) {
            // 辿り着けない場合パス
            if (dp[i][j] == INF) continue;

            if (j + Ai <= M) {
                dp[i+1][j+Ai] = min(dp[i+1][j+Ai], dp[i][j] + 1);
            }
            dp[i+1][j] = min(dp[i+1][j], dp[i][j]);
        }
    }
    // print_array(dp);
    cout << (dp[N][M] != INF ? dp[N][M] : -1) << endl;
}

// WA
// #include <bits/stdc++.h>
// using namespace std;

// int INF = 1000000;

// int main() {
//     int N, M; cin >> N >> M;
//     vector<vector<int>> dp(N+1, vector<int>(M+1, INF));
//     dp[0][0] = 0;

//     for (int i = 0; i < N; i++) {
//         int Ai; cin >> Ai;
//         for (int j = 0; j <= M; j++) {
//             if (dp[i][j] < INF) {
//                 if (j + Ai <= M) {
//                     dp[i+1][j+Ai] = min(dp[i+1][j+Ai], dp[i][j] + 1);
//                 }
//                 dp[i+1][j] = dp[i][j];  // <- ここが問題だったか
//             }
//         }
//     }
//     cout << (dp[N][M] != INF ? dp[N][M] : -1) << endl;
// }