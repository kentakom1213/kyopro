/*
# [0-1 Knapsack Problem](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B&lang=jp)

## result
- AC
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int MAX_N = 111, MAX_W = 101010;
int N, W;
int v[MAX_N], w[MAX_W];
int memo[MAX_N][MAX_W];

int pack(int i, int rem) {
    if (memo[i][rem] != -1) return memo[i][rem];
    if (i == N) return 0;

    // i個めを選ばない
    int ans = pack(i+1, rem);
    // i個めを選ぶ
    if (w[i] <= rem) {
        chmax(ans, v[i] + pack(i+1, rem - w[i]));
    }
    return memo[i][rem] = ans;
}

int main() {
    FILL(memo, -1);
    cin >> N >> W;
    rep(i, N) cin >> v[i] >> w[i];

    cout << pack(0, W) << endl;
}
