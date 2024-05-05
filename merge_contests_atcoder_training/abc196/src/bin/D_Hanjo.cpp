//                D - Hanjo                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc196/tasks/abc196_d

// AC
// ------------------------------------

/*
## 方針
- A枚の2x1畳を敷き詰める
- その後、B枚の1x1畳の敷き詰め方は一意に定まる

### 定義
- 0: 未定
- 1: 2x1

### 埋め方
```
0110
1000
1011
```

*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int H, W, A, B;

void dfs(vector<vector<bool> > &F, int i, int a, ll &ans) {
    if (a == 0) {
        ans++;
        return;
    } else if (i == H*W) {
        return;
    }

    // 置かない場合
    dfs(F, i+1, a, ans);

    // 置く場合
    int r=i/W, c=i%W;
    if (!F[r][c]) {
        F[r][c] = 1;

        // 横向きに置く
        if (c+1<W && !F[r][c+1]) {
            F[r][c+1] = 1;
            dfs(F, i+1, a-1, ans);
            F[r][c+1] = 0;
        }

        // 縦向きに置く
        if (r+1<H && !F[r+1][c]) {
            F[r+1][c] = 1;
            dfs(F, i+1, a-1, ans);
            F[r+1][c] = 0;
        }
        F[r][c] = 0;
    }
}

int main() {
    cin >> H >> W >> A >> B;

    vector<vector<bool>> F(H, vector(W, false));

    ll ans = 0;
    dfs(F, 0, A, ans);

    cout << ans << endl;
}
