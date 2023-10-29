//            C - 民族大移動
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc024/tasks/abc024_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    ll N, D, K; cin >> N >> D >> K;
    vector<point> LR(D), ST(K);
    for (auto& [l, r] : LR) cin >> l >> r;
    for (auto& [s, t] : ST) cin >> s >> t;

    
}