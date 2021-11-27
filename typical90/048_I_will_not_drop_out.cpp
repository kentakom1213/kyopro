//      048 - I will not drop out（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_av

// 解説
// https://twitter.com/e869120/status/1396960059796582400/photo/1

// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define SLICE(A, b, e) A.begin()+b, A.begin()+e

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
    int N, K; cin >> N >> K;
    vector<int> A(N), B(N);
    rep(i, N) cin >> A[i] >> B[i];

    // 上界を求める
    vector<int> points(2*N);
    rep(i, N) {
        points[i] = B[i];
        points[2*i] = A[i] - B[i];
    }

    sort(ALL(points), greater<int>());
    // 上界
    ll upper = accumulate(SLICE(points, 0, K), 0);
    
    cout << upper << endl;
}