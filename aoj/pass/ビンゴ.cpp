//                   ビンゴ                   
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0537

// パスで
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }

constexpr int mod = 100000;

// 分割数をあらかじめ計算
int p[3030][55];

int main() {
    // 分割数もどき
    p[0][0] = 1;
    rep(i, 0, 3030) {
        rep(j, 1, 55) {
            if (j <= i) {
                p[i][j] = (p[i-j][j] + p[i][j-1]) % mod;
            } else {
                p[i][j] = p[i][j-1];
            }
        }
    }

    while (true) {
        int N, M, S; cin >> N >> M >> S;
        if (N == 0 && M == 0 && S == 0) return 0;
        int k = N*N, n = S - k*(k+1)/2, m = M - k;

    }
}
