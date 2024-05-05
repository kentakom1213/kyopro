//                 F - LCS
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_f
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

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

template <typename T>
bool chmax(T &a, const T& b) {
  if (a < b) {
    a = b;  // aをbで更新
    return true;
  }
  return false;
}

int main() {
    string s, t; cin >> s >> t;
    int ls = s.size(), lt = t.size();

    vector<vector<int>> dp(ls+1, vector<int>(lt+1, 0));
    
    for (int i = 0; i < ls; i++) {
        for (int j = 0; j < lt; j++) {
            if (s[i] == t[j]) {
                chmax(dp[i+1][j+1], dp[i][j] + 1);
            }
            chmax(dp[i+1][j+1], dp[i][j+1]);
            chmax(dp[i+1][j+1], dp[i+1][j]);
        }
    }
    print_array(dp);

    // 文字列を作る
    
}