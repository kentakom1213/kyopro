//         056 - Lucky Bag（★5）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bd
// ----------------------------------------

// 2^100 ~= 10^30 だから、全探索は不可能
// dp配列に情報を保存

#include <bits/stdc++.h>
using namespace std;
typedef unsigned long long ll;
#define rep(i, n) for (ll i = 0; i < (ll)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<ll>> name(h, vector<ll>(w, v));
typedef pair<ll, ll> vec2;
typedef vector<vector<ll>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    ll N, S; cin >> N >> S;
    vector<ll> A(N), B(N);
    rep(i, N) cin >> A[i] >> B[i];

    vector<vector<string>> dp(N+1, vector<string>(S+1, ""));
    dp[0][0] = "X";

    for (ll i = 0; i < N; i++) {
        for (ll j = 0; j < S; j++) {
            if (dp[i][j] == "") continue;

            // Aを追加
            if (j + A[i] <= S) {
                dp[i+1][j+A[i]] = dp[i][j] + 'A';  // Aのときは足さない
            }
            // Bを追加
            if (j + B[i] <= S) {
                dp[i+1][j+B[i]] = dp[i][j] + 'B';  // Bのときは2^iを足す
            }
        }
    }

    string res = dp[N][S];
    if (res != "") {
        cout << res.substr(1, N) << endl;;
    }
    else {
        cout << "Impossible" << endl;
    }
}
