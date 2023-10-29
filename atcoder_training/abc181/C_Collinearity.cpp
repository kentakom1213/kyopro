//             C - Collinearity
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc181/tasks/abc181_c

// AC
// ----------------------------------------

// 異なる3点を選択する場合 O(n^3)
// N <= 10^2 だから全探索かな

// 同一直線上にある条件
//     (yj - yi) / (xj - xi) == (yk - yi) / (xk - xi)
// <=> (yj - yi) * (xk - xi) == (yk - yi) * (xj - xi)


#include <iostream>
#include <vector>
using namespace std;

int main() {
    int N; cin >> N;
    vector<pair<int, int>> P(N);
    for (auto& [x, y] : P) cin >> x >> y;

    bool isYes = false;
    for (int i = 0; i < N; i++) {
        for (int j = i+1; j < N; j++) {
            for (int k = j+1; k < N; k++) {

                auto [xi, yi] = P[i];
                auto [xj, yj] = P[j];
                auto [xk, yk] = P[k];

                // 同一直線上にある条件
                bool isCollinear = (yj - yi) * (xk - xi) == (yk - yi) * (xj - xi);
                
                isYes |= isCollinear;
            }
        }
    }
    cout << (isYes ? "Yes" : "No") << endl;
}