//         D - 8 Puzzle on Graph
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc224/tasks/abc224_d
// ----------------------------------------

/* comment
## 方針
- コマの状態を順列と考えて保存
- 最短経路問題に帰着する
- 9! = 362880 だから問題なし 
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

int M;
vector<vector<int>> G;
vector<int> P;

int main() {
    cin >> M;
    G.assign(9, vector<int>());
    P.assign(8, 0);

    rep(i, M) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }
    rep(i, 8) {
        cin >> P[i];
        P[i]--;
    }
}
