//            B - K個のケーキ
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/code-festival-2016-qualc/tasks/codefestival_2016_qualC_b

// これでいいのか
// AC
// ----------------------------------------

// 貪欲にいけそう

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
    int k, t; cin >> k >> t;
    vector<int> A(t);
    rep(i, t) cin >> A[i];

    sort(ALL(A), greater<int>());
    int res = A[0] - accumulate(A.begin()+1, A.end(), 0);

    cout << max(res-1, 0) << endl;
}