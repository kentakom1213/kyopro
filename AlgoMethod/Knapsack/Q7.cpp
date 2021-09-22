//        問題 7：個数制限付き部分和問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/313

// 解説
// https://algo-method.com/tasks/313/editorial

// 要復習

// AC (????)
// ----------------------------------------

// 漸化式
// dp[i+1][j] := i番目までの整数の中から選んで、総和をjとするときの選んだA[i]の個数の最小値

//              整数Aiを選ばない場合
// dp[i+1][j] = min| 0   (dp[i][j] != INF)
//                 | INF (dp[i][j] == INF)
//              整数Aiを新たに選ぶ場合
//                 | 1   (dp[i][j-Ai] != INF)
//                 | INF (dp[i][j-Ai] == INF)
//              整数Aiを追加で選ぶ場合
//                 | dp[i+1][j-Ai] + 1 (dp[i+1][j-Ai] <= B[i])
//                 | INF               (dp[i+1][j-Ai] >= B[i])

#include <bits/stdc++.h>
using namespace std;

const int INF = 1 << 29;

int main() {
    int N, M; cin >> N >> M;

    vector<int> A(510), B(10010);
    for (int i = 0; i < N; ++i) cin >> A[i] >> B[i];

    vector<vector<int>> dp(N+1, vector<int>(M+1, INF));
    dp[0][0] = 0;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j <= M; j++) {
            if(dp[i][j] < INF) dp[i+1][j] = 0;
            if (j >= A[i]) {
                if (dp[i][j-A[i]] < INF) {
                    dp[i+1][j] = min(dp[i+1][j], 1);
                }
                if (dp[i+1][j-A[i]] < B[i]) {
                    dp[i+1][j] = min(dp[i+1][j], dp[i+1][j-A[i]] + 1); 
                }
            }
        }
    }
    if (dp[N][M] < INF) cout << "Yes" << endl;  
    else cout << "No" << endl; 
}
