//             D - RGB Triplets            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc162/tasks/abc162_d
// ----------------------------------------

// 包除原理

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int N;
int R[101010], G[101010], B[101010];  // iより右にあるR,G,Bの数
string S;

int main() {
    cin >> N;
    cin >> S;

    // 右側から累積和
    for (int i=N-1; i >= 0; i--) {
        R[i] = R[i+1];
        G[i] = G[i+1];
        B[i] = B[i+1];

        switch (S[i]) {
            break; case 'R':
                R[i] = R[i+1] + 1;
            break; case 'G':
                G[i] = G[i+1] + 1;
            break; case 'B':
                B[i] = B[i+1] + 1;
        }
    }

    // rep(i, 0, N) printf("%2d ", R[i]); cout << "\n";
    // rep(i, 0, N) printf("%2d ", G[i]); cout << "\n";
    // rep(i, 0, N) printf("%2d ", B[i]); cout << "\n";

    ll ans = 0;

    // 2重ループ
    rep(i, 0, N) {
        rep(j, i+1, N) {
            ll tmp = 0;
            int k = j + (j-i);
            if (S[i] == 'R' && S[j] == 'G' || S[i] == 'G' && S[j] == 'R') {
                tmp += B[j+1];
                if (k < N && S[k] == 'B') tmp--;
            }
            if (S[i] == 'G' && S[j] == 'B' || S[i] == 'B' && S[j] == 'G') {
                tmp += R[j+1];
                if (k < N && S[k] == 'R') tmp--;
            }
            if (S[i] == 'B' && S[j] == 'R' || S[i] == 'R' && S[j] == 'B') {
                tmp += G[j+1];
                if (k < N && S[k] == 'G') tmp--;
            }
            ans += tmp;

            // if (tmp) printf("i:%d, j:%d, k:%d -> tmp:%lld\n", i+1, j+1, k+1, tmp);
        }
    }

    cout << ans << endl;

}
