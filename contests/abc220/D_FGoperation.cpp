
// f(a, b), g(a, b) (0 <= a, b <= 9) にかんしてのテーブルを作っておく？
// 計算量 10^5 * 10 = 10^6

#include <bits/stdc++.h>
using namespace std;

template <typename T>
void print_array(vector<vector<T>>& array) {
  int H = array.size();
  int W = array.at(0).size();
  
  cout << "{" << endl;
  for (int i = 0; i < H; i++) {

    cout << "  {";
    for (int j = 0; j < W; j++) {
      if (j < W - 1) cout << array.at(i).at(j) << ", ";
      else cout << array.at(i).at(j);
    }
    cout << "}," << endl;
  }
  cout << "}" << endl;
}


// // main
// int main() {
//     // input
//     int N; cin >> N;
//     vector<int> A(N);
//     for (int i = 0; i < N; i++) cin >> A[i];

//     // dpテーブル
//     vector<vector<int>> dp(N-1, vector<int>(10, 0));
//     dp[0][ (A[0] + A[1]) % 10 ]++;
//     dp[0][ (A[0] * A[1]) % 10 ]++;

//     // 処理
//     for (int i = 0; i < N-2; i++) {
//         for (int j = 0; j < 10; j++) {
//             if (not dp[i][j]) continue;
//             cout << j << " " << A[i+1] << " " << (j + A[i]) % 10 << " " << (j * A[i]) % 10 << endl;
//             dp[i+1][ (A[i+1] + j) % 10 ]++;
//             dp[i+1][ (A[i+1] * j) % 10 ]++;
//         }
//     }

//     print_array(dp);

// }


// AC (解説)
#include <bits/stdc++.h>
using namespace std;

int MOD = 998244353;

// main
int main() {
    // input
    int N; cin >> N;
    vector<int> A(N);
    for (int i = 0; i < N; i++) cin >> A[i];

    // dpテーブル
    vector<vector<int>> dp(N, vector<int>(10, 0));
    dp[0][A[0]]++;

    // 処理
    for (int i = 0; i < N-1; i++) {
        for (int j = 0; j < 10; j++) {
            if (not dp[i][j]) continue;

            dp[i+1][ (A[i+1] + j) % 10 ] = (dp[i][j] + dp[i+1][ (A[i+1] + j) % 10 ]) % MOD;
            dp[i+1][ (A[i+1] * j) % 10 ] = (dp[i][j] + dp[i+1][ (A[i+1] * j) % 10 ]) % MOD;

        }
    }

    for (int i = 0; i < 10; i++) {
        cout << dp[N-1][i] << endl;
    }

}
