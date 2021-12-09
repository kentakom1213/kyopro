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

    initArray(dp, N+1, S+1, 0);
    dp[0][0] = 1;

    for (ll i = 0; i < N; i++) {
        for (ll j = 0; j < S; j++) {
            if (!dp[i][j]) continue;

            // Aを追加
            if (j + A[i] <= S) {
                dp[i+1][j+A[i]] = dp[i][j];  // Aのときは足さない
            }
            // Bを追加
            if (j + B[i] <= S) {
                dp[i+1][j+B[i]] = dp[i][j] + (1 << (i+1));  // Bのときは2^iを足す
            }
        }
    }

    ll res = dp[N][S];
    if (res) {
        string combi;

        printf("res:%lld, S:%lld\n", res, S);

        for (int i = 1; i <= N; i++) {
            combi.push_back( ((res >> i) & 1) ? 'B' : 'A' );
        }
        cout << combi << endl;
    }
    else {
        cout << "Impossible" << endl;
    }
}
