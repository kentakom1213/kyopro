// B - 1%
// -------
// 問題
// https://atcoder.jp/contests/abc165/tasks/abc165_b
// -------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll X; cin >> X;
    ll x = 100;
    int cnt = 0;
    while (x < X) {
        x = x + x/100; 
        cnt++;
    }

    cout << cnt << endl;
}

