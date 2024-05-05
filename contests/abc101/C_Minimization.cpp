//             C - Minimization            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc101/tasks/arc099_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int N, K;

int div_ceil(int a, int b) {
    return (a+b-1) / b;
}

int main() {
    cin >> N >> K;
    // おおえれば良い
    cout << div_ceil(N-1, K-1) << endl;
}
