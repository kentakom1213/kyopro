//        033 - Not Too Bright（★2）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ag

// AC
// ----------------------------------------

// ↓コーナーケースでWA
#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int H, W; cin >> H >> W;

    if (H != 1 && W != 1) {
        int y = ceil((double)H / 2);
        int x = ceil((double)W / 2);

        cout << y * x << endl;
    }
    else cout << H * W << endl;
}