//        028 - Cluttered Paper（★4） 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ab
// ----------------------------------------

// 見るからに2次元のいもす法
// 正方形だから縦横は考慮しなくてもいい

#include <bits/stdc++.h>
using namespace std;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define defArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;

constexpr int max_size = 1100;

int main() {
    int N; cin >> N;
    defArray(value, max_size, max_size, 0);
    defArray(sum, max_size, max_size, 0);

    rep(i, N) {
        int lx, ly, rx, ry;
        cin >> lx >> ly >> rx >> ry;

        value[lx][ly]++;
        value[lx][ry]--;
        value[rx][ly]--;
        value[rx][ry]++;
    }

    // 縦に累積和を取る
    rep(i, max_size) {
        rep(j, max_size) {
            if (i == 0) sum[i][j] = value[i][j];
            else sum[i][j] = sum[i-1][j] + value[i][j];
        }
    }

    // 横に累積和を取る
    rep(i, max_size) {
        rep(j, max_size) {
            if (j == 0) continue;
            sum[i][j] = sum[i][j-1] + sum[i][j];
        }
    }

    // 各マスに関して和を求める
    vector<int> res(N+1, 0);
    rep(i, max_size) {
        rep(j, max_size) {
            res[sum[i][j]]++;
        }
    }

    rep(i, N) cout << res[i+1] << endl;
}