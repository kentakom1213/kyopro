//            C - Multiple Clocks
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc070/tasks/abc070_c

// AC (解説)
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

// オーバーフロー
ll gcd(ll a, ll b) {
    if (a < b) swap(a, b);
    while (a > 1) {
        a %= b;
        swap(a, b);
    }
    return b;
}

// うまくいく
ll gcd(ll a, ll b) {
    if (b == 0) return a;
    return gcd(b, a%b);
}

ll lcm(ll a, ll b) {
    ll g = gcd(a, b);
    return a / g * b;
}

int main() {
    int N; cin >> N;

    ll res = 1;
    for (int i = 0; i < N; i++) {
        ll t; cin >> t;
        res = lcm(res, t);
    }
    cout << res << endl;
}