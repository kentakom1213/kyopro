//            問題 3：部分和問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/309

// 謎のエラー

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M; cin >> N >> M;
    vector<vector<bool>> dp(101, vector<bool>(10000, false));
    dp[0][0] = true;

    for (int i = 0; i < N; i++) {
        int Ai; cin >> Ai;
        for (int j = 0; j <= M; j++) {
            if (dp[i][j]) {
                if (dp[i][j] + Ai <= M) {
                    dp[i+1][j+Ai] = true;
                }
                dp[i+1][j] = true;
            }
        }
    }
    cout << (dp[N][M] ? "Yes" : "No") << endl;
}


// RE ???
// #include <bits/stdc++.h>
// using namespace std;

// int main() {
//     int N, M; cin >> N >> M;
//     vector<vector<bool>> dp(N+1, vector<bool>(M+1, false));
//     dp[0][0] = true;
//     for (int i = 0; i < N; i++) {
//         int Ai; cin >> Ai;
//         for (int j = 0; j <= M; j++) {
//             if (dp[i][j]) {
//                 if (dp[i][j] + Ai <= M) {
//                     dp[i+1][j+Ai] = true;
//                 }
//                 dp[i+1][j] = true;
//             }
//         }
//     }
//     cout << (dp[N][M] ? "Yes" : "No") << endl;
// }