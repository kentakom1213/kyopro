//         C - Walking Takahashi
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc175/tasks/abc175_c

// シンプルな記述がミスを減らす

// AC (解説)
// ----------------------------------------

// #include <bits/stdc++.h>
// using namespace std;
// typedef long long ll;

// int main() {
//     ll x, k, d; cin >> x >> k >> d;

//     x = abs(x);
//     ll nearest = x % d, rest = k - x / d;

//     if (k * d < x) cout << x - k * d << endl;
//     else if (rest % 2 == 0) {
//         cout << nearest << endl;
//     } else {
//         cout << abs(d - nearest) << endl;
//     }
// }


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll x, k, d; cin >> x >> k >> d;

    x = abs(x);
    ll straight = min(k, x / d);
    k -= straight;
    x -= straight * d;

    if (k % 2 == 0) {
        cout << x << endl;
    } else {
        cout << d - x << endl;
    }
}