//        020 - Log Inequality（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_t

// AC
// ----------------------------------------

// c^b ≤ 13^17 = 8650415919381337933 < 9×10^18
// long long で足りるらしい

// この類の問題はpythonに限る
// a, b, c = map(int, input().split())
// print("Yes" if a < c**b else "No")

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll a, b, c; cin >> a >> b >> c;
    cout << (a < powl(c, b) ? "Yes" : "No") << endl;
}