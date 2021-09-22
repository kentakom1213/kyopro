//      問題 8：最長共通部分列 (LCS) 問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/314

// 復習になった

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    string S, T; cin >> S >> T;
    vector<vector<int>> dp(S.size()+1, vector<int>(T.size()+1, 0));

    for (int i = 0; i < S.size(); i++) {
        for (int j = 0; j < T.size(); j++) {
            if (S[i] == T[j]) dp[i+1][j+1] = dp[i][j] + 1;
            else dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1]);
        }
    }
    // print_array(dp);
    cout << dp[S.size()][T.size()] << endl;
}