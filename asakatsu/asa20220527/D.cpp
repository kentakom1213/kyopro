// https://atcoder.jp/contests/arc041/tasks/arc041_b

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

int N, M, F[505][505], ans[505][505];

int main() {
    cin >> N >> M;
    rep(i, 0, N) {
        string row; cin >> row;
        rep(j, 0, M) {
            F[i][j] = row[j] - '0';
        }
    }
    
    rep(i, 1, N-1) {
        rep(j, 1, M-1) {
            int around_min = min(
                min(F[i+1][j], F[i-1][j]),
                min(F[i][j+1], F[i][j-1])
            );
            ans[i][j] = around_min;

            F[i+1][j] -= around_min;
            F[i-1][j] -= around_min;
            F[i][j+1] -= around_min;
            F[i][j-1] -= around_min;
        }
    }

    rep(i, 0, N) {
        rep(j, 0, M) {
            cout << ans[i][j];
        }
        cout << "\n";
    }
}