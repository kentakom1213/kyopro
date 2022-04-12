//             B - Flip Digits             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/agc049/tasks/agc049_b
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int MAX_SIZE = 505050;

int N;
string S, T;
bool Sxor[MAX_SIZE];

int main() {
    cin >> N >> S >> T;

    // 累積xorを取る
    Sxor[0] = 0;
    rep(i, N) Sxor[i+1] = Sxor[i] ^ (S[i] - '0');

    rep(i, N+1) cerr << Sxor[i];
    cerr << endl;
}
