
// 一般的な方法（試し割り法）
// TLE
// https://algo-method.com/submissions/174410

// #include <bits/stdc++.h>
// using namespace std;
// typedef long long ll;
// #define rep(i, n) for (int i = 0; i < (int)(n); i++)

// typedef long long ll;
// bool is_prime(ll n) {
//     if (n == 1) return false;
//     if (n == 2) return true;
//     for (ll i = 2; i*i <= n; i++) {
//         if (n % i == 0) return false;
//     }
//     return true;
// }

// int main() {
//     int N; cin >> N;
//     rep(i, N) {
//         ll a; cin >> a;
//         bool isOK = is_prime(a);
//         cout << (isOK ? "Yes" : "No") << endl;
//     }
// }

// ミラー・ラビン素数判定法
#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
using namespace std;

// a^n % m を高速に求める
template<typename T>
T pow_mod(T a, T n, T m) {
    
}

int main() {

}