//             D - aab aba baa 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc202/tasks/abc202_d

// 参考
// https://atcoder.jp/contests/abc202/editorial/1860
// ----------------------------------------

// A, B <= 30 よりすべての順列は 60!/2*30! = 1.5685009237285812e+49
// 全探索は不可能
// メモ化再帰を使うらしいです。
// 空文字列Sにa, bを追加するしていく組み合わせ

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

template <typename T>
void print_array(vector<vector<T>>& array) {
  int H = array.size();
  int W = array.at(0).size();
  
  cout << "{" << endl;
  for (int i = 0; i < H; i++) {

    cout << "  {";
    for (int j = 0; j < W; j++) {
      if (j < W - 1) cout << array.at(i).at(j) << ", ";
      else cout << array.at(i).at(j);
    }
    cout << "}," << endl;
  }
  cout << "}" << endl;
}

constexpr int MAX = 30;
vector<vector<ll>> dp(MAX+1, vector<ll>(MAX+1, 0));

string find_str(int A, int B, ll K) {
    if (A == 0) return string(B, 'b');
    if (B == 0) return string(A, 'a');
    if (K == dp[A - 1][B]) {
        return string("a") + find_str(A-1, B, K);
    }
    else {
        return string("b") + find_str(A, B - 1, K - dp[A - 1][B]);
    }
}

int main() {
    int A, B; cin >> A >> B;
    ll K; cin >> K;

    dp[0][0] = 1;
    for (int i = 0; i <= A; i++) {
        for (int j = 0; j <= B; j++) {
            if (i > 0) dp[i][j] += dp[i-1][j];
            if (j > 0) dp[i][j] += dp[i][j-1];
        }
    }
    print_array(dp);
}