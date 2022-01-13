//     D - Prediction and Restriction
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc149/tasks/abc149_d

// 参考
// https://drken1215.hatenablog.com/entry/2020/04/25/022700

// AC (解説)
// ----------------------------------------

// dpで解く

// #include <bits/stdc++.h>
// using namespace std;
// typedef long long ll;

// template <typename T>
// void print_vector(vector<T>& vec) {
//   cerr << "[ ";
//   for (int i = 0; i < vec.size(); i++) {
//     if (i < vec.size() - 1) cerr << vec.at(i) << " ";
//     else cerr << vec.at(i);
//   }
//   cerr << " ]" << endl;
// }

// int main() {
//     int N, K, R, S, P; cin >> N >> K >> R >> S >> P;
//     string T; cin >> T;

//     // 勝った時の点数を定義
//     map<char, int> point{ {'r', P}, {'s', R}, {'p', S} };

//     vector<ll> dp(N, 0);  // dp[i] := i回目までの最大の得点
//     for (int i = 0; i < N; i++) {
//         // 最初のK回
//         if (i == 0) dp[i] = point[T[i]];
//         else if (i < K) dp[i] = dp[i-1] + point[T[i]];
        
//         // 更新
//         else {
//             if (T[i] == T[i-K]) {
//                 dp[i] = max(
//                     dp[i-1],  // 前回勝つ
//                     dp[i-K] - point[T[i-K]] + point[T[i]]  // 今回勝つ
//                 );
//             }
//             else dp[i] = dp[i-1] + point[T[i]];  // 勝つ
//         }
//         printf("dp[%d] = %lld\n", i, dp[i]);
//     }

//     print_vector(dp);
// }

// 解説
// K回飛ばしで考えると、「1つ前と同じ手が出せない」という問題に帰着できる

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N, K, R, S, P; cin >> N >> K >> R >> S >> P;
    string T; cin >> T;

    // 勝った時の点数を定義
    map<char, int> point{ {'r', P}, {'s', R}, {'p', S} };

    ll res = 0;
    for (int k = 0; k < K; k++) {
        bool last = false;  // 2回に1回足すためのフラグ
        for (int i = k; i < N; i += K) {
            if (i >= K && T[i-K] == T[i] && last) last = false;
            else res += point[T[i]], last = true;
        }
    }
    cout << res << endl;
}