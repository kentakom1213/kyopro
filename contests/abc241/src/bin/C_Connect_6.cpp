//              C - Connect 6              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc241/tasks/abc241_c
// ----------------------------------------

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
    int N;
    cin >> N;
    vector<string> S(N);
    rep(i, 0, N) cin >> S[i];

    bool isOK = false;

    // 横, 縦
    rep(i, 0, N) {
        rep(j, 0, N-6+1) {
            int h=0, v=0;
            rep(k, 0, 6) {
                h += (S[i][j+k] == '.');  // 横
                v += (S[j+k][i] == '.');  // 縦
            }
            isOK |= (h <= 2 || v <= 2);
        }
    }

    // 斜め
    rep(i, 0, N) {
        rep(j, 0, N) {
            int lu=6, ru=6;  // 左上, 右上
            rep(k, 0, 6) {
                if (i+6 <= N && j+6 <= N) {
                    lu -= (S[i+k][j+k] == '#');
                }
                if (i-5 >= 0 && j+6 <= N) {
                    ru -= (S[i-k][j+k] == '#');
                }
            }
            isOK |= (lu <= 2 || ru <= 2);
        }
    }

    cout << (isOK ? "Yes" : "No") << endl;
}
