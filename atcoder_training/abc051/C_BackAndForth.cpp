//           C - Back and Forth
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc051/tasks/abc051_c

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int sx, sy, tx, ty;
    cin >> sx >> sy >> tx >> ty;
    int X = tx - sx, Y = ty - sy;

    cout << string(Y, 'U') << string(X, 'R') << string(Y, 'D') << string(X+1, 'L');
    cout << string(Y+1, 'U') << string(X+1, 'R') << "DR" << string(Y+1, 'D') << string(X+1, 'L') << 'U' << endl;
}