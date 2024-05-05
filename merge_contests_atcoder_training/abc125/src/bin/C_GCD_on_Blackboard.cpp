//         C - GCD on Blackboard
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc125/tasks/abc125_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

ll gcd(ll a, ll b) {
    if (b == 0) return a;
    return gcd(b, a%b);
}

int main() {
    int N; cin >> N;
    vector<ll> A(N);
    rep(i, N) cin >> A[i];

    // 累積GCD
    vector<ll> L(N), R(N);
    L[0]=A[0], R[N-1]=A[N-1];
    rep(i, N-1) {
        L[i+1] = gcd(L[i], A[i+1]);
        R[N-i-2] = gcd(R[N-i-1], A[N-i-2]);
    }

    // i番目を除いた数列のGCDを求める
    ll ans = 0; 
    rep(i, N) {
        if (i==0) chmax(ans, R[i+1]);
        if (i==N-1) chmax(ans, L[N-2]);
        else {
            chmax(ans, gcd(L[i-1], R[i+1]));
        }
    }

    cout << ans << endl;
}
