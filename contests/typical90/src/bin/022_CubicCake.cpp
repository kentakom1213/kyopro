//            022 - Cubic Cake
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_v

// こういう問題の考察を速くしていきたい

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll A, B, C; cin >> A >> B >> C;

    // 最大公約数を求める -> 積を求める
    ll gcd_abc = gcd(A, gcd(B, C));
    A -= gcd_abc, B -= gcd_abc, C -= gcd_abc;
    A /= gcd_abc, B /= gcd_abc, C /= gcd_abc;

    cout << A + B + C << endl;
}