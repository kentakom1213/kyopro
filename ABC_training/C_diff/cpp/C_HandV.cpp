//               C - H and V
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc173/tasks/abc173_c

// AC
// ----------------------------------------

// 行・列の選び方は全部で 2^(H+W) 通り
// H, W <= 6 より 2^(H+W) <= 262144  全探索が間に合う

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int H, W, K; cin >> H >> W >> K;

    vector<string> field(H);
    for (int i = 0; i < H; i++) cin >> field[i];

    int res = 0;
    for (int i = 0; i < (1 << (H+W)); i++) {
        vector<int> rows(H, 1), cols(W, 1);
        // 行の削除
        for (int j = 0; j < H; j++) {
            if ((i >> j) & 1) {
                // row[j] を削除
                rows[j] = 0;
            }
        }
        // 列の削除
        for (int j = 0; j < W; j++) {
            if ((i >> (j+H)) & 1) {
                // cols[j] を削除
                cols[j] = 0;
            }
        }

        // print_vector(rows);
        // print_vector(cols);
        // cerr << endl;

        // 条件を満たしているかどうかを調べる
        int count_black = 0;
        for (int y = 0; y < H; y++) {
            for (int x = 0; x < W; x++) {
                bool is_black = field[y][x] == '#';
                bool isnot_red = rows[y] && cols[x];
                if (is_black && isnot_red) count_black++;
            }
        }
        if (count_black == K) res++;
    }
    cout << res << endl;
}