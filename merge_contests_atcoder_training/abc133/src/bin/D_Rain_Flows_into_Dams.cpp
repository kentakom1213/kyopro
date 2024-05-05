//         D - Rain Flows into Dams        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc133/tasks/abc133_d
// ----------------------------------------

/*
# 方針
Nが奇数であることを利用する。

x1/2 + x2/2 = A1
x2/2 + x3/2 = A2
 ...

↓
x1 + x2 = 2*A1
x2 + x3 = 2*A2
 ...
xN + x1 = 2*AN

↓
    (x1+x2)-(x2+x3)+(x3+x4)- ... +(xN+x1) -> 2*x1
<=> x1 = A1 - A2 + A3 - A4 + ... + AN
*/

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N; cin >> N;
    vector<int> A(N);
    for (int i = 0; i < N; i++) cin >> A[i];

    // x1 を求める
    ll x = 0;
    for (int i = 0; i < N; i++) {
        x += (i&1 ? -1 : 1) * A[i];
    }

    cout << x << " ";

    // x2以降を求める
    for (int i = 0; i < N-1; i++) {
        x = 2*A[i] - x;
        cout << x << " ";
    }
    cout << "\n";
}
