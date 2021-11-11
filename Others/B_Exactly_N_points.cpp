//         B - Exactly N points
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/cf16-final/tasks/codefestival_2016_final_b

// AC
// ----------------------------------------

// #include <bits/stdc++.h>
#include <iostream>
using namespace std;
typedef long long ll;

int main() {
    ll N; cin >> N;

    // Nより大きい最小の三角数を求める
    auto triangle = [=](ll n) -> ll { return n * (n+1) / 2; };
    ll l = 1, r = N;
    while ((r - l) > 1) {
        ll mid = (l + r) / 2;
        if (triangle(mid) < N) l = mid;
        else r = mid;
    }

    // rが求める数である
    ll diff = triangle(r) - N;
    for (ll i = 1; i <= r; i++) {
        if (i != diff) {
            cout << i << endl;
        }
    }

}