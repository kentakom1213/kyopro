//          079 - Two by Two（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ca

// AC
// ----------------------------------------

// 貪欲に解いてみる

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int H, W; cin >> H >> W;
    vector<vector<int>> diff(H, vector(W, 0));

    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            int a; cin >> a;
            diff[i][j] -= a;  // 先に引いておく
        }
    }
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            int b; cin >> b;
            diff[i][j] += b;
        }
    }

    // 貪欲に処理
    ll cnt = 0;
    int h_render = H-2+1 , w_render = W-2+1;  // 処理の回数
    for (int i = 0; i < h_render; i++) {
        for (int j = 0; j < w_render; j++) {
            // int min_val = min({diff[i][j], diff[i+1][j], diff[i][j+1], diff[i+1][j+1]});
            // int max_val = max({diff[i][j], diff[i+1][j], diff[i][j+1], diff[i+1][j+1]});
            // if (min_val > 0) {
            //     diff[i][j] -= min_val;
            //     diff[i+1][j] -= min_val;
            //     diff[i][j+1] -= min_val;
            //     diff[i+1][j+1] -= min_val;
            // }

            // if (max_val < 0) {
            //     diff[i][j] -= max_val;
            //     diff[i+1][j] -= max_val;
            //     diff[i][j+1] -= max_val;
            //     diff[i+1][j+1] -= max_val;
            // }

            int lu = diff[i][j];
            cnt += abs(lu);
            diff[i][j] -= lu;
            diff[i+1][j] -= lu;
            diff[i][j+1] -= lu;
            diff[i+1][j+1] -= lu;
        }
    }

    bool isOK = true;
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            isOK &= diff[i][j] == 0;
        }
    }

    if (isOK) {
        cout << "Yes" << endl;
        cout << cnt << endl;
    }
    else {
        cout << "No" << endl;
    }
}