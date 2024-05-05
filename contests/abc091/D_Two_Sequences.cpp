//            D - Two Sequences
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc091/tasks/arc092_b
// ----------------------------------------

#include <iostream>
#include <vector>
using namespace std;
#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")

using namespace std;

int a[220000], b[220000];

int main() {
  int n;
  cin >> n;
  for (int i = 0; i < n; i++) {
    cin >> a[i];
  }
  for (int i = 0; i < n; i++) {
    cin >> b[i];
  }

  // 2重ループで探索
  int ans = 0;
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < n; j++) {
      ans ^= a[i] + b[j];
    }
  }
  cout << ans << endl;
  return 0;
}
