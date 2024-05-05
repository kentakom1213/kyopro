//               B - 縞模様
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc020/tasks/arc020_2

// AC
// ----------------------------------------

// 色2つの選び方は 10C2 = 45 通り
// それぞれについて2通りずつ考えられる
// n <= 100 であるから合計で 45*2*100 <= 10000で十分間に合う


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
    int n, c; cin >> n >> c;
    vector<int> A(n);
    rep(i, n) cin >> A[i];

    int min_change = 10000;

    for (int c1 = 1; c1 <= 10; c1++) {
        for (int c2 = c1+1; c2 <= 10; c2++) {
            // cnt1: c1,c2,c1... で塗り分ける場合
            // cnt2: c2,c1,c2... で塗り分ける場合
            int cnt1 = 0, cnt2 = 0;
            rep(i, n) {
                if (i % 2 == 0) {
                    cnt1 += A[i] != c1;
                    cnt2 += A[i] != c2;
                }
                else {
                    cnt1 += A[i] != c2;
                    cnt2 += A[i] != c1;
                }
            }

            chmin(min_change, cnt1);
            chmin(min_change, cnt2);
        }
    }

    cout << min_change * c << endl;
}