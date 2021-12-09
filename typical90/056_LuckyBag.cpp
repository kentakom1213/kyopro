//         056 - Lucky Bag（★5）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bd

// AC
// ----------------------------------------

// 2^100 ~= 10^30 だから、全探索は不可能
// dp配列に情報を保存

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N, S; cin >> N >> S;
    vector<int> A(N), B(N);
    rep(i, N) cin >> A[i] >> B[i];

    initArray(dp, N+1, S+1, 0);
    dp[0][0] = 1;

    // 普通のナップサック問題
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < S; j++) {
            if (!dp[i][j]) continue;

            // Aを追加
            if (j + A[i] <= S) {
                dp[i+1][j+A[i]] = dp[i][j];
            }
            // Bを追加
            if (j + B[i] <= S) {
                dp[i+1][j+B[i]] = dp[i][j];
            }
        }
    }

    // 解なし
    if (dp[N][S] == 0) {
        cout << "Impossible" << endl;
        return 0;
    }

    // dp復元
    string res;
    int now_sum = S;
    for (int i = N-1; i >= 0; i--) {
        if (now_sum - A[i] >= 0 && dp[i][now_sum - A[i]]) {
            // printf("now_sum - A[i] = %d - %d = %d\n", now_sum, A[i], now_sum-A[i]);
            res.push_back('A');
            now_sum -= A[i];
        }
        else {
            // printf("now_sum - B[i] = %d - %d = %d\n", now_sum, B[i], now_sum-B[i]);
            res.push_back('B');
            now_sum -= B[i];
        }
    }

    reverse(ALL(res));
    cout << res << endl;
}

