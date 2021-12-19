//         029 - Long Bricks（★5）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ac
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int W, N; cin >> W >> N;
    vector<int> L(N), R(N);
    rep(i, N) cin >> L[i] >> R[i];

    
}