//           C - Changing Jewels           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc260/tasks/abc260_c
// ----------------------------------------

/*
# 漸化式
R_n = R_{n-1} + X B_n
B_n = R_{n-1} + Y B_{n-1}
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

ll N, X, Y;
ll RED[20], BLUE[20];

ll red(int n);
ll blue(int n);

ll red(int n) {
    if (n == 1) return 0;
    if (BLUE[n] == -1) blue(n);
    if (RED[n-1] == -1) red(n-1);
    return RED[n] = RED[n-1] + X * BLUE[n];
}

ll blue(int n) {
    if (BLUE[n-1] == -1) blue(n-1);
    if (RED[n-1] == -1) red(n-1);
    return BLUE[n] = RED[n-1] + Y * BLUE[n-1];
}

int main() {
    cin >> N >> X >> Y;

    // dp配列の初期化
    FILL(RED, -1);
    FILL(BLUE, -1);
    RED[1] = 0;
    BLUE[1] = 1;

    // solve
    cout << red(N) << endl;
}