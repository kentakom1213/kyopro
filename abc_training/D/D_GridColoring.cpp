//           D - Grid Coloring
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc069/tasks/arc080_b

// PythonでAC
// ----------------------------------------

// 配置の仕方を考える
// 1 6 7
// 2 5 8
// 3 4 9

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int H, W, N; cin >> H >> W >> N;
    vector<int> A(N);
    for (auto& a : A) cin >> a;

    int res[H][W];
    int row = 0;
    bool isEven;
    for (int i = 0; i < N; i++) {
        isEven = (i / H) % 2 == 0;  // 偶数行目->左から、奇数行目->右から

        if (isEven) {
            res[row][i/H] = i+1;
            row++;
        } else {
            res[row][i/H] = i+1;
            row--;
        }
    }

    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            cout << res[i][j] << " ";
        }
        cout << endl;
    }
}