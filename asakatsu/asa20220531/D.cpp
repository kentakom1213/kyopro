// https://atcoder.jp/contests/abc182/tasks/abc182_e

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int F[2000][2000];
bool ans[2000][2000];

int main() {
    int H, W, N, M;
    cin >> H >> W >> N >> M;
    rep(i, 0, N) {
        int a, b; cin >> a >> b;
        F[a-1][b-1] = 1;
    }
    rep(i, 0, M) {
        int c, d; cin >> c >> d;
        F[c-1][d-1] = 2;
    }

    // 左->右
    rep(i, 0, H) {
        bool is_lighted = false;
        rep(j, 0, W) {
            if (F[i][j] == 1) {
                is_lighted = true;
            } else if (F[i][j] == 2) {
                is_lighted = false;
            }
            ans[i][j] |= is_lighted;
        }
    }

    // 右->左
    rep(i, 0, H) {
        bool is_lighted = false;
        for (int j=W-1; j>=0; j--) {
            if (F[i][j] == 1) {
                is_lighted = true;
            } else if (F[i][j] == 2) {
                is_lighted = false;
            }
            ans[i][j] |= is_lighted;
        }
    }

    // 上->下
    rep(j, 0, W) {
        bool is_lighted = false;
        rep(i, 0, H) {
            if (F[i][j] == 1) {
                is_lighted = true;
            } else if (F[i][j] == 2) {
                is_lighted = false;
            }
            ans[i][j] |= is_lighted;
        }
    }

    // 下->上
    rep(j, 0, W) {
        bool is_lighted = false;
        for (int i=H-1; i>=0; i--) {
            if (F[i][j] == 1) {
                is_lighted = true;
            } else if (F[i][j] == 2) {
                is_lighted = false;
            }
            ans[i][j] |= is_lighted;
        }
    }
    // // 表示
    // rep(i, 0, H) {
    //     rep(j, 0, W) {
    //         cout << F[i][j] << " ";
    //     }
    //     cout << endl;
    // }
    // cout << endl;

    // rep(i, 0, H) {
    //     rep(j, 0, W) {
    //         cout << ans[i][j] << " ";
    //     }
    //     cout << endl;
    // }

    // カウント
    int out = 0;
    rep(i, 0, H) rep(j, 0, W) out += ans[i][j];
    cout << out << endl;
}