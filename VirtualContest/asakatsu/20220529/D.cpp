// https://atcoder.jp/contests/abc191/tasks/abc191_c

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

int main() {
    int H, W;
    cin >> H >> W;
    vector<string> F(H);
    rep(i, 0, H) cin >> F[i];

    int ans = 0;
    rep(i, 0, H-1) {
        rep(j, 0, W-1) {
            ans += (
                (F[i][j] == '#')
                + (F[i][j+1] == '#')
                + (F[i+1][j] == '#')
                + (F[i+1][j+1] == '#')
            ) & 1;
        }
    }
    cout << ans << endl;
}
