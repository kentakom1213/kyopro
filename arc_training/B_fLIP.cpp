//                B - fLIP
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/code-festival-2017-quala/tasks/code_festival_2017_quala_b
// ----------------------------------------

// できる場合
// - Nの倍数
// - Mの倍数
// - N+M-2
// - 2N+M-4, N+2M-4
// - ...
// - aN+bM-2ab

// したがって、
// K = aN + bM - 2ab を満たす(a,b)が存在するか判定すれば良い
// => 4ab - 2aN - 2bM = -2K
// => (2a - M)(2b - N) = NM - 2K

// ex1)
// NM - 2K = 2*2 - 2*2 = 0
// (a, b) = (0, 1) など

// ex2)
// NM - 2K = 2*2 - 2*1 = 2
// -> a, bは存在しない

// ex3)
// NM - 2K = 3*5 - 2*8 = -1
// (a, b) = (1, 3) など

// 0 <= a <= N, 0 <= b <= M について全探索すればよい

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = a; i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N, M, K; cin >> N >> M >> K;
    int nm_2k = N*M - 2*K;
    rep(a, 0, N+1) {
        rep(b, 0, M+1) {
            int prod = (2*a - N) * (2*b - M);
            if (prod == nm_2k) {
                cout << "Yes" << endl;
                cerr << "a:" << a << ", b:" << b << endl;
                return 0;
            }
        }
    }
    cout << "No" << endl;
}
