//        C - X: Yet Another Die Game
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc068/tasks/arc068_a
//
// AC
// ----------------------------------------

// x <= 10^15

// 7: 6 -> 5
// 6 -> 5 を繰り返せばいいのでは

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
    ll x; cin >> x;
    ll div11 = x / 11;  // 5->6で減らし続ける回数
    ll mod11 = x % 11;  // あまり
    cout << div11*2 + (mod11 == 0 ? 0 :(mod11 <= 6 ? 1 : 2)) << endl;
}