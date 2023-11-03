// 014 - We Used to Sing a Song Together（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_n
// ----------------------------------------

// 単純にマッチングするとO(n^2)
// 配列をソート -> DP

// dp[i] := A[:i]、B[:i]までを用いたときの、最小の不便さ

// 更新
// dp[0] = abs(A[0] - B[0])  // これより小さい不便さは存在しない
// dp[i] = min(
//     dp[i-1] + abs(A[i] - B[i]),  // A[i], B[i]をマッチング
//     dp[i]
// )

// 違うじゃん...

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N; cin >> N;
    vector<ll> A(N), B(N);
    rep(i, N) cin >> A[i];
    rep(i, N) cin >> B[i];
    sort(ALL(A));
    sort(ALL(B));

    ll res = 0;
    rep(i, N) {
        res += abs(A[i] - B[i]);
    }

    cout << res << endl;
}