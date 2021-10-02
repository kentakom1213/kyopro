//          Q5. ゴールドバッハ予想 (2)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/331

// AC (解説)
// ----------------------------------------

// #include <bits/stdc++.h>
// using namespace std;
// template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
// template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

// int main() {
//     int N; cin >> N;
//     vector<int> primes(N+1, 1);  // 0 ~ N の配列
//     primes[0] = primes[1] = 0;

//     // 素数テーブルを作成
//     for (int i = 2; i*i <= N; i++) {
//         if (!primes[i]) continue;
//         for (int j = 2; i*j <= N; j++) {
//             primes[i*j] = 0;
//         }
//     }

//     // DPで検索
//     vector<vector<int>> dp(N+1, vector<int>(N+1, 0));
//     dp[0][0] = 1;
//     for (int i = 0; i < N; i++) {
//         for (int p = i; p < N; p++) {
//             if (primes[p]) {
//                 if (i + p <= N) {
//                     printf("%d, %d\n", i, p);
//                     dp[i+1][i+p] = dp[i][i] + dp[i][p];
//                 }
//             }
//             // printf("%d, %d\n", dp[i+1][p], dp[i][p]);
//             chmax(dp[i+1][p], dp[i][p]);
//         }
//     }

//     print_array(dp);
// }

// 2つという条件があるからdpを使うとかえってややこしい

// 解説
#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; cin >> N;
    vector<int> primes(N+1, 1);  // 0 ~ N の配列
    primes[0] = primes[1] = 0;

    // 素数テーブルを作成
    for (int i = 2; i*i <= N; i++) {
        if (!primes[i]) continue;
        for (int j = 2; i*j <= N; j++) {
            primes[i*j] = 0;
        }
    }

    int res = 0;
    for (int p = 2; p <= N/2; p++) {
        int other = N - p;
        if (primes[p] && primes[other]) res++;
    }
    cout << res << endl;
}