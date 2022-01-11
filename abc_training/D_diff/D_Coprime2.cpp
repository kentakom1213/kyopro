//              D - Coprime 2
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc215/tasks/abc215_d

// AC
// ----------------------------------------

// Aの約数を列挙する         N * O(sqrt(n))
// エラトステネスの篩を用いる  O(nloglog(n))

#include <bits/stdc++.h>
using namespace std;

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
    int N, M; cin >> N >> M;

    // 約数列挙
    set<int> factors;
    for (int k = 0; k < N; k++) {
        int a; cin >> a;
        for (int i = 2; i*i <= a; i++) {
            while (a % i == 0) {
                factors.insert(i);
                a /= i;
            }
        }
        if (a != 1) factors.insert(a);
    }

    // test
    for (auto i = factors.begin(); i != factors.end(); i++) {
        cerr << *i << " ";
    }
    cerr << endl;

    // エラトステネスの篩
    vector<int> res(M+1, 1);
    res[0] = 0;

    for (auto i = factors.begin(); i != factors.end(); i++) {
        int i_val = *i;
        for (int j = 1; i_val*j <= M; j++) {
            res[i_val*j] = 0;
        }
    }
    // test
    print_vector(res);

    // 表示
    cout << accumulate(res.begin(), res.end(), 0) << endl;
    for (int i = 0; i <= M; i++) {
        if (res[i]) cout << i << endl;
    }
}
