//      006 - Smallest Subsequence（★5）     
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_f
// ----------------------------------------

/*

## 方針
- 辞書順最小は貪欲法

### 前計算
c[i][j] := i文字目の右にある文字jの中で、最も左側にあるもののindex

*/

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

int main() {
    int N, K; cin >> N >> K;
    string S; cin >> S;
    
    // c[i][j] := i文字目の右にある文字jの中で、最も左側にあるもののindex
    vector<vector<int>> c(N+1, vector<int>(26, N+1));
    for (int i = 0; i < N; i++) {
        int ch = S[i] - 'a';
        c[i][ch] = i;
    }
    for (int i = N-1; i >= 0; i--) {
        for (int j = 0; j < 26; j++) {
            chmin(c[i][j], c[i+1][j]);
        }
    }

    // cout << "     "; for (int i=0; i<26; i++) printf(" %c  ", 'a'+i); cout << "\n";
    // for (int i=0; i<N; i++) {
    //     printf("%2d | ", i);
    //     for (int j=0; j<26; j++) {
    //         printf("%2d  ", c[i][j]);
    //     }
    //     cout << "\n";
    // }

    // 貪欲にとる
    string ans = "";

    int i=0, k=0;  // S[i]でk文字を構成する
    while (k < K) {
        for (int j = 0; j < 26; j++) {
            if (N - c[i][j] >= K - k) {
                // 文字jを追加
                ans.push_back('a' + j);
                k++;
                i = c[i][j] + 1;
                break;
            }
        }
    }

    cout << ans << endl;
}
