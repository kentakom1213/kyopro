//      063 - Monochromatic Subgrid（★4） 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bk

// AC
// ----------------------------------------

// H <= 8であるから、 行に関して全探索: 1 << 8 = 256通り
// 横に見ていって、同じ列に同じ数字が書いてある場合にmapに追加

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
    int H, W; cin >> H >> W;
    initArray(field, H, W, 0);
    rep(i, H) rep(j, W) cin >> field[i][j];

    int res = 1;

    // 行の組み合わせをえらぶ
    for (int i = 1; i < (1 << H); i++) {
        map<int, int> num_count;  // 行に含まれる色をカウント
        int row_cnt = 0;  // 行の数

        // 列
        for (int w = 0; w < W; w++) {
            bool is_same = true;  // その列が同じ数字で構成されているか
            int num_col = 0;  // 各列の数字

            // 全ての行で同じ色かどうか判定
            for (int h = 0; h < H; h++) {
                if ((i >> h) & 1) {
                    if (w == 0) row_cnt++;  // 行数をカウント

                    if (num_col) {
                        is_same &= field[h][w] == num_col;
                    }
                    else {
                        num_col = field[h][w];
                    }
                }
            }

            if (is_same) num_count[num_col]++;
        }
        
        int zantei = -1;
        for (auto [num, cnt] : num_count) {
            chmax(zantei, cnt);
        }

        // 答えの書き換え
        chmax(res, zantei * row_cnt);
    }

    cout << res << endl;
}