//                I - Coins
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_i
// ----------------------------------------

// dp[i][j] := coins[:i]からj個を選んだ時の積の最大値

// dp[0][j] = max| coins[0]
//               |

// dp[i][j] = max| dp[i-1][j]  // 選ばない場合
//               | dp[i-1][j-1] * coins[i]  // 選ぶ場合

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

template <typename T>
void print_array(vector<vector<T>>& array) {
    int H = array.size();

    cerr << "{" << endl;
    for (int i = 0; i < H; i++) {
        int W = array.at(i).size();
        cerr << "  {";
        for (int j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
}

int main() {
    int N; cin >> N;
    vector<int> coins(N);
    for (auto& p : coins) cin >> p;

    vector<vector<double>> dp(N, vector<double>(N+1, 0));
    for (int i = 1; i < N; i++) {
        for (int j = 0; j <= N; j++) {
            if (i == 0) dp[i][j] = coins[0];
            else {
                dp[i][j] = max(
                    dp[i-1][j],
                    dp[i-1][j-1] * coins[i]
                );
            }
        }
    }

    print_array(dp);
}