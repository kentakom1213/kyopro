//    002 - Encyclopedia of Parentheses
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_b
// ----------------------------------------

// "(" -> +1, ")" -> -1
// 上のように定義すると、正しい括弧列の総和は0
// 括弧列の累積和は常に負でない

// 例
// (()())    : 1,2,1,2,1,0
// ((()))    : 1,2,3,2,1,0

// この条件を満たす +1, -1 の組み合わせを列挙する
// 順列の関係から +1と-1を入れ替える

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
    int N; cin >> N;
    if (N & 1) cout << endl;  // 奇数の正しいかっこ列は存在しない
    else {
        vector<int> perm(N);
        for (int i = 0; i*2 < N; i++) perm[i] = -1, perm[i+N/2] = 1;
        
        while (perm[0] != 1) {
            for (int i = 0; i < N; i++) {
                cout << (perm[i] == -1 ? "(" : ")");
            }
            cout << endl;

            next_permutation(perm.begin(), perm.end());
        }
    }
}

// この方針でいくとちょっと難しそう