//          C - Digital Graffiti
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc191/tasks/abc191_c

// 参考
// https://atcoder.jp/contests/abc191/editorial/612

// 難しいねえ

// AC (解説)
// ----------------------------------------

// 何角形であるかを判定するという問題

// .....
// .###.
// .#...
// .##..
// .....

// ↑これは9角形になる

// 方針
// 角をn=4とする、スタートの#(一番上の左)を決める。
// #について、右に#があれば

// 解説
// # ではなく隙間の点に注目
// 周囲4マスに関して#が1または3個あるとき、頂点である。


#include <bits/stdc++.h>
using namespace std;

int main() {
    int H, W;
    cin >> H >> W;
    vector<string> F(H);
    for (int i = 0; i < H; i++) cin >> F[i];

    int kado = 0;
    for (int i = 0; i < H-1; i++) {
        for (int j = 0; j < W-1; j++) {
            vector<bool> cells = {
                F[i][j] == '#',
                F[i][j+1] == '#',
                F[i+1][j] == '#',
                F[i+1][j+1] == '#'
            };

            int isKado = accumulate(cells.begin(), cells.end(), 0) % 2;
            kado += isKado;
        }
    }
    cout << kado << endl;
}