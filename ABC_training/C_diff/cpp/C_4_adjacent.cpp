//            C - 4 - adjacent
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc069/tasks/arc080_a

// よくないねえ

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;

    int four = 0, two = 0;
    for (int i = 0; i < N; i++) {
        int a; cin >> a;
        if (a == 0) continue;
        if (a % 4 == 0) four++;
        else if (a % 2 == 0) two++;
    }
    bool isOK = four * 2 + 1 + two / 2 * 2  - N >= 0;
    cerr << four << " " << two << " " << four * 2 + 1 + two / 2 * 2 << endl;
    cout << (isOK ? "Yes" : "No") << endl;
}