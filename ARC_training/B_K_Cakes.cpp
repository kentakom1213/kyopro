//            B - K個のケーキ
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/code-festival-2016-qualc/tasks/codefestival_2016_qualC_b
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
    queue<vec2> cakes;
    rep(i, t) {
        cakes.push(make_pair(i, A[i]));
    }

    int now = -1, res = 0;
    while (!cakes.empty()) {
        auto [cake, num] = cakes.front();
        // printf("<%d, %d>\n",cake, num);
        cakes.pop();

        if (now == cake) res++;
        now = cake;
        if (num > 1) cakes.push(make_pair(cake, num-1));
    }

    cout << res << endl;
}