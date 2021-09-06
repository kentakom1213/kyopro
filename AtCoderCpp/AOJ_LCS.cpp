//       Longest Common Subsequence
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5857553#2

// C++ 書きづらい

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

template <typename T>
void print_array(vector<vector<T> >& array) {
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

int getLCS(string x, string y) {
  int lx = x.size();
  int ly = y.size();
  vector<vector<int> > DP(lx+1, vector<int>(ly+1, 0));

  for (int i = 0; i < lx; i++) {
    for (int j = 0; j < ly; j++) {
      if (x[i] == y[j]) DP[i+1][j+1] = DP[i][j] + 1;
      else {
        DP[i+1][j+1] = max(
          DP[i+1][j],
          DP[i][j+1]
        );
      };
    }
  }

  // print_array(DP);
  return DP[lx][ly];
}


int main() {
  int N;
  cin >> N;

  vector<string> data1(N), data2(N);
  for (int i = 0; i < N; i++) {
    cin >> data1[i] >> data2[i];
  }

  for (int i = 0; i < N; i++) {
    cout << getLCS(data1[i], data2[i]) << endl;
  }
}