//              D - Stamp
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc185/tasks/abc185_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()

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
    ll n, m;
    cin >> n >> m;
    vector<ll> A(m+2, 0), diff(m);
    rep(i, m) {
        int a; cin >> a;
        A[i+1] = a - 1;
    }
    sort(ALL(A));
    

    rep(i, m) {
        diff[i] = A[i+1] - A[i];
    }

    print_vector(A);
    print_vector(diff);
}