//             C - Subarray Sum            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/keyence2020/tasks/keyence2020_c
// ----------------------------------------

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
    int N, K, S; cin >> N >> K >> S;

    for (int i = 0; i < K; i++) cout << S << " ";
    for (int i = 0; i < N-K; i++) cout << (S == 1 ? 2 : S-1) << " ";
    cout << "\n";
}
