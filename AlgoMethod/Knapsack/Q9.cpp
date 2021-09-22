//      問題 9：最小コスト弾性マッチング問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/316

// 今日は諦めよう
// ----------------------------------------

// 漸化式
// dp[i+1][j+1] := AiまでとBiまでの弾性マッチングの最小コスト
// dp[i+1][j+1] を dp[i][j], dp[i+1][j], dp[i][j+1] を用いて表す

// dp[i+1][j+1] = min| dp[i][j] + costs[i][j]
//                   | dp[i+1][j] + costs[i+1][j]

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M; cin >> N >> M;
    vector<vector<int>> costs(N, vector<int>(M));
    for (int i = 0; i < N; i++) for (int j = 0; j < M; j++) cin >> costs[i][j];

    
}