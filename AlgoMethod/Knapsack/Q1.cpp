//             問題 1：最大和問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/307

// AC
// ----------------------------------------

// #include <bits/stdc++.h>
// using namespace std;

// // べつにこれでも良いのだが、
// int main() {
//     int N; cin >> N;
//     int res = 0;
//     for (int i = 0; i < N; i++) {
//         int Ai; cin >> Ai;
//         if (Ai > 0) res += Ai;
//     }
//     cout << res << endl;
// }


// DP的にはこう
#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; cin >> N;
    vector<int> dp(N+1);
    dp[0] = 0;

    for (int i = 0; i < N; i++) {
        int Ai; cin >> Ai;
        dp[i+1] = max(
            dp[i],
            dp[i] + Ai
        );
    }
    cout << dp[N] << endl;
}