//               C - H and V
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc173/tasks/abc173_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

template <typename T>
void print_vector(vector<T>& vec) {
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << "\n";
    else cerr << vec.at(i);
  }
  cerr << endl;
}

int main() {
    int H, W, K; cin >> H >> W >> K;
    vector<string> field(H);
    for (int i = 0; i < H; i++) cin >> field[i];

    print_vector(field);
}