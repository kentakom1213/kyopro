//         B - Grid Repainting 4
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc131/tasks/arc131_b

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<long long>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

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

const vector<vec2> direct = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

int main() {
    ll H, W; cin >> H >> W;
    initArray(field, H, W, 0);
    rep(i, H) {
        string row; cin >> row;
        rep(j, W) {
            if (row[j] != '.') field[i][j] = row[j] - '0';
        }
    }
    
    // 貪欲に選ぶ
    rep(i, H) rep(j, W) if (field[i][j] == 0) {
        // 残っている色を計算
        vector<int> color(6, 1);
        for (auto [di, dj] : direct) {
            int ni = i+di, nj = j+dj;
            if (0 <= ni && ni < H && 0 <= nj && nj < W && field[ni][nj]) {
                int ncol = field[ni][nj];
                color[ncol] = 0;
            }
        }
        // 残っている色がある場合は埋める
        for (int c = 1; c <= 5; c++) {
            if (color[c]) field[i][j] = c;
        }
    }

    rep(i, H) {
        rep(j, W) cout << field[i][j];
        cout << endl;
    }
}
