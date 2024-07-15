//         Largest prime factor
// ----------------------------------------
// 問題
// https://projecteuler.net/problem=3
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll N = 600851475143;

    // factorize
    vector<ll> ans;
    for (ll i = 2; i*i <= N; i++) {
        while (N % i == 0) {
            ans.push_back(i);
            N /= i;
        }
    }
    if (N != 1) ans.push_back(N);

    for (auto x : ans) {
        cout << x << " ";
    }
    cout << endl;
}
