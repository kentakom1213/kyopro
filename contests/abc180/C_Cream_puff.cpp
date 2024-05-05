//              C - Cream puff
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc180/tasks/abc180_c

// 参考
// https://atcoder.jp/contests/abc180/editorial/181

// AC
// ----------------------------------------

// 約数を列挙
// 10^12回ループを回すのは厳しい
// 素因数分解 -> bitで全列挙

// #include <iostream>
// #include <vector>
// using namespace std;
// typedef long long ll;

// vector<pair<int, int>> factoring_pair(ll n) {
//     vector<pair<int, int>> res;
//     for (int i = 2; i*i <= n; i++) {
//         int count = 0;
//         while (n % i == 0) {
//             count++;
//             n /= i;
//         }
//         res.push_back({i, count});
//     }
//     if (n != 1) res.push_back({n, 1});
//     return res;
// }

// vector<int> factoring(ll n) {
//     vector<int> res;
//     for (int i = 2; i*i <= n; i++) {
//         while (n % i == 0) {
//             res.push_back(i);
//             n /= i;
//         }
//     }
//     if (n != 1) res.push_back(n);
//     return res;
// }

// int main() {
//     int N; cin >> N;
//     auto fac = factoring(N);
    
//     for (int i = 0; i < (1 << fac.size()); i++) {
//         ll res = 1;
//         for (int j = 0; j < fac.size(); j++) {
//             if ((i >> j) & 1) {
//                 res *= fac[j];
//             }
//         }
//         cout << res << endl;
//     }
// }
// 上だと重複を排除できない


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll N; cin >> N;
    set<ll> res;
    for (int i = 1; i*i <= N; i++) {
        if (N % i == 0) {
            res.insert(i);
            res.insert(N/i);
        }
    }
    for (auto x: res) cout << x << endl;
}