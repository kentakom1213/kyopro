//         C - Exam and Wizard
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/keyence2019/tasks/keyence2019_c
// ----------------------------------------

// 解説
// https://img.atcoder.jp/keyence2019/editorial.pdf


#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;

int main() {
    ll N; cin >> N;
    vector<ll> A(N), B(N);
    rep(i, N) cin >> A[i];
    rep(i, N) cin >> B[i];

    ll S = 0;  // A[i] - B[i]の和
    ll cnt = 0;  // A[i] < B[i]となる個数
    rep(i, N) {
        S += (A[i] - B[i]);
        cnt += (A[i] < B[i]);
    }

    if (S < 0) cout << -1 << endl;
    else if (cnt == 0) cout << 0 << endl;
    else cout << cnt << endl;
}