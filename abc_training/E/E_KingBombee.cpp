//            E - King Bombee
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc244/tasks/abc244_e
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

template <typename T>
void print_array(vector<vector<T>>& array) {
    int H = array.size();

    cerr << "{" << endl;
    for (int i = 0; i < H; i++) {
        int W = array.at(i).size();
        cerr << "  {";
        for (int j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
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
    // 頂点Sから辺をK回通って頂点Tへ行く方法は何通り？
    initArray(dp, K+1, N, 0);
    rep(j, N) dp[0][j] = j == S;

    rep(i, K) {
        for (auto [u, v] : edges) {
            dp[i+1][v] = (dp[i+1][v] + dp[i][u]) % mod;
            dp[i+1][u] = (dp[i+1][u] + dp[i][v]) % mod;
        }
    }

    print_array(dp);
}