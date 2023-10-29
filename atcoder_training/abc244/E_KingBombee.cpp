//            E - King Bombee
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc244/tasks/abc244_e

// 解説AC
// ----------------------------------------

/* comment

## 方針
- 「頂点Sから辺をK回通って頂点Tへ行く方法は何通り？」という問題に帰着し、DPでとく
- 偶数回という条件に関しては後で考える

## dp
### 初期条件
```
dp[i][j] := (頂点Sから辺をi回通って頂点jに行く方法の数)
```

### 漸化式
```
dp[0][j] = | 1 (j == S)
           | 0 (j != S)

dp[i+1][j] = sum_{k in adj(j)} dp[i][k]
```

*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

void addMod(int &a, const int b, int mod) {
    a += b;
    a %= mod;
}

int main() {
    int N, M, K, S, T, X;
    cin >> N >> M >> K >> S >> T >> X;
    S--, T--, X--;

    vector<vec2> edges(M);
    for (auto &[u, v] : edges) {
        cin >> u >> v;
        u--, v--;
    }

    // dp
    vector dp(K+1, vector(N, vector(2, 0)));
    dp[0][S][0] = 1;

    rep(i, K) {
        for (auto [u, v] : edges) {
            for (int s : {0, 1}) {
                addMod(dp[i+1][v][s^(v==X)], dp[i][u][s], mod);
                addMod(dp[i+1][u][s^(u==X)], dp[i][v][s], mod);
            }
        }
    }

    cout << dp[K][T][0] << endl;
}