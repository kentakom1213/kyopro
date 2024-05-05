//             C - Knight Fork
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc239/tasks/abc239_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    vector<vec2> OK = {
        {1, 1},
        {0, 2}, {2, 0},
        {1, 3}, {3, 1},
        {3, 3},
        {2, 4}, {4, 2},
        {0, 4}, {4, 0}
    };

    int x1, y1, x2, y2; cin >> x1 >> y1 >> x2 >> y2;
    int dx = abs(x1-x2), dy = abs(y1-y2);

    for (auto [u, v] : OK) {
        if (u==dx && v==dy) {
            cout << "Yes" << endl;
            return 0;
        }
    }
    cout << "No" << endl;
}
