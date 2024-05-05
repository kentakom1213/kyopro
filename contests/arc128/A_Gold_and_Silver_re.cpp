//          A - Gold and Silver
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc128/tasks/arc128_a

// むずすぎる

// AC (解説)
// ----------------------------------------

// i回目における金の最大量をG[i]とおく。
// G[i+2] = G[i] * A[i] / A[i+1]
// よって、A[i] > A[i+1] であればお得

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int n; cin >> n;
    vector<ll> A(n), ans(n);
    rep(i, n) cin >> A[i];

    rep(i, n-1) {
        if (A[i] > A[i+1]) {
            ans[i] ^= 1;
            ans[i+1] ^= 1;
        }
    }
    
    rep(i, n) cout << ans[i] << " ";
    cout << endl;
}
