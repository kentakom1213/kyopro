//               D - Cooking
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc204/tasks/abc204_d

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

template <typename T>
int last_item_index(vector<T> &v) {
    int res = -1;
    for (int i = 0; i < v.size(); i++) {
        if (v[i]) res = i;
    }
    return res;
}

int main() {
    int N; cin >> N;
    vector<int> T(N);
    for (int i = 0; i < N; i++) cin >> T[i];

    int sum_time = accumulate(T.begin(), T.end(), 0);
    int half = sum_time / 2;

    vector<vector<bool>> dp(N+1, vector<bool>(half+1, 0));
    dp[0][0] = true;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j <= half; j++) {
            if (dp[i][j]) {
                if (j + T[i] <= half) dp[i+1][j+T[i]] = true;
                dp[i+1][j] = true;
            }
        }
    }
    int last_item = last_item_index(dp[N]);
    cout << max(last_item, sum_time - last_item) << endl;    
}
