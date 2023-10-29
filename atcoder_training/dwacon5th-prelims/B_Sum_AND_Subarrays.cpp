//          B - Sum AND Subarrays          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dwacon5th-prelims/tasks/dwacon5th_prelims_b

// pass
// ----------------------------------------

/* comment
- 連続する部分列 -> O(n^2)
- 任意の個数の部分列の和に関して、それらの論理積の最大値
- 部分列は N(N+1)/2 個存在

### K = N(N+1)/2のとき
- 自明に全ての和の論理積
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

int main() {
    int N, K; cin >> N >> K;
    ll A[N];
    rep(i, N) cin >> A[i];

    // 累積和
    ll S[N+1]; S[0] = 0;
    rep(i, N) S[i+1] = S[i] + A[i];


}
