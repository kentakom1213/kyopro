// B - Kagami Mochi
// ----------------
// 問題
// https://atcoder.jp/contests/abc085/tasks/abc085_b
//
// AC
// ----------------

#include <iostream>
#include <set>
using namespace std;

int main() {
    int n; cin >> n;
    set<int> kagami;
    for (int i = 0; i < n; i++) {
        int mochi; cin >> mochi;
        kagami.insert(mochi);
    }

    cout << kagami.size() << endl;
}
