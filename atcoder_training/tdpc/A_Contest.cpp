//              A - コンテスト
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/tdpc/tasks/tdpc_contest

// チート級の速さ (8ms)

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N;
    cin >> N;
    vector<int> P(N);
    for (int i = 0; i < N; i++) cin >> P[i];

    int MAX = 10000;
    vector<vector<int> > dp(N+1, vector<int>(MAX, false));
    dp[0][0] = true;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < MAX; j++) {
            if (dp[i][j]) {
                dp[i+1][j+P[i]] = true;
                dp[i+1][j] = true;
            }
        }
    }

    cout << count(dp[N].begin(), dp[N].end(), true) << endl;

}