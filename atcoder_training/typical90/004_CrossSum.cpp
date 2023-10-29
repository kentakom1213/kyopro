//         004 - Cross Sum（★2）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_d

// 結構ギリギリだった

// AC
// ----------------------------------------

// 4方向から二次元累積和を取る？

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main() {
    int H, W; cin >> H >> W;
    vector<vector<int>> A(H, vector<int>(W, 0));
    for (int i = 0; i < H; i++) for (int j = 0; j < W; j++) cin >> A[i][j];

    vector<int> H_sum(H, 0), W_sum(W, 0);
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            H_sum[i] += A[i][j];
            W_sum[j] += A[i][j];
        }
    }
    
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            cout << H_sum[i] + W_sum[j] - A[i][j] << " ";
        }
        cout << endl;
    }

}