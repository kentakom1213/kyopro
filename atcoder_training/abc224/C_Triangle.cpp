//             C - Triangle?
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc224/tasks/abc224_c

// AC
// ----------------------------------------

// 選び方は n(n-1)(n-2) / 6
// 300^3 / 6 = 4500000 だからギリ間に合いそう？

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    vector<pair<int, int>> points(N);
    for (auto& [x, y] : points) cin >> x >> y;

    int res = 0;
    for (int i = 0; i < N; i++) {
        for (int j = i+1; j < N; j++) {
            for (int k = j+1; k < N; k++) {
                auto [ax, ay] = points[i];
                auto [bx, by] = points[j];
                auto [cx, cy] = points[k];

                // printf("(%d, %d), (%d, %d), (%d, %d)\n", ax, ay, bx, by, cx, cy);
                // 直線でないかの判定
                bool is_on_a_line = (cy - ay) * (bx - ax) == (cx - ax) * (by - ay);
                if (!is_on_a_line) res++;
            }
        }
    }
    cout << res << endl;
}