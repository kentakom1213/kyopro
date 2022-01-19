//              E - Dist Max
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc178/tasks/abc178_e
// ----------------------------------------

// 「45度回転」っていうのを使うらしい

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int n; cin >> n;
    vector<vec2> points(n);
    rep(i, n) cin >> points[i].first >> points[i].second;

    
}