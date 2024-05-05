//        020 - Log Inequality（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_t
// ----------------------------------------

// log_2 a < b log_2 c

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<long long>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    ll a, b, c; cin >> a >> b >> c;
    cout << (a < powl(c, b) ? "Yes" : "No") << endl;
}
