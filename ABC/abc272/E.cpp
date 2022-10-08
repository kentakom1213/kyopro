/*

## 方針
- ソートによって大小関係が変化することはない
- 二分探索？

## 実装
- n回目の操作で要素iには n*i が加算されている

 */

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N, M; cin >> N >> M;
    vector<ll> A(N);
    for (auto &a : A) cin >> a;
    sort(ALL(A));

    
}