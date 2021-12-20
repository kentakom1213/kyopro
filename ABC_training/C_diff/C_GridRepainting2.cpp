//          C - Grid Repainting 2
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc096/tasks/abc096_c

// AC
// ----------------------------------------

// 1 <= H, W <= 50

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int H, W; cin >> H >> W;
    char field[H][W];
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            cin >> field[i][j];
        }
    }

    bool isOK = true;
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            if (field[i][j] != '#') continue;
            bool okcell = false;
            if (0 < i) if (field[i-1][j] == '#') okcell = true;
            if (0 < j) if (field[i][j-1] == '#') okcell = true;
            if (i < H-1) if (field[i+1][j] == '#') okcell = true;
            if (j < W-1) if (field[i][j+1] == '#') okcell = true;
            isOK &= okcell;
        }
    }

    cout << (isOK ? "Yes" : "No") << endl;
}