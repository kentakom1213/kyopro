//         008 - AtCounter（★4）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_h

// なかなか頑張った

// AC
// ----------------------------------------

// dpで解く？
// dp[i][j] := Sのi番目までの文字を使ってatcoderのj番目までを表す組合せ

// dp[i+1][j+1] = dp[i][j]                   // S[i] != atcoder[j]
// dp[i+1][j+1] = dp[i-1][j] + dp[i-1][j+1]  // S[i] == atcoder[j]

// #define _GLIBCXX_DEBUG
#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
const int MOD = 1000000007;

int main() {
    int N; cin >> N;
    string S; cin >> S;
    string atcoder = "atcoder";

    // dpテーブル
    vector<vector<ll>> dp(N+1, vector<ll>(atcoder.size()+1, 0));
    dp[0][0] = 1;
    
    for (int i = 0; i < N; i++) {
        for (int j = 0; j <= atcoder.size(); j++) {
            // printf("(i, j) = (%d, %d) = (%c, %c)\n", i, j, S[i], atcoder[j-1]);
            // 一致する場合
            if (S[i] == atcoder[j-1]) {
                dp[i+1][j] = dp[i][j-1] + dp[i][j];
                dp[i+1][j] %= MOD;
            }
            // 一致しない場合は下ろす
            else {
                dp[i+1][j] = dp[i][j];
            }
        }
    }
    cout << dp[N][7] << endl;
}