//         D - Practical Skill Test        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc089/tasks/abc089_d
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

typedef pair<int, int> P;

int H, W, D, A[333][333], Q, S[101010];
P idx[101010];

int main() {
    cin >> H >> W >> D;
    rep(i, 0, H) rep(j, 0, W) {
        cin >> A[i][j];
        idx[A[i][j]] = {i, j};
    }

    // i -> i+D に移動するときのコストの累積和をとっておく
    rep(k, 0, H*W-D+1) {
        auto [i, j] = idx[k];
        auto [x, y] = idx[k+D];
        S[k+D] = S[k] + abs(x-i) + abs(y-j);
    }

    cin >> Q;
    while (Q--) {
        int l, r; cin >> l >> r;
        cout << S[r] - S[l] << endl;
    }
}
