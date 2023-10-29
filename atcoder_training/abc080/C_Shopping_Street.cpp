//           C - Shopping Street           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc080/tasks/abc080_c
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const ll MAX_N = 111;
ll N, F[MAX_N][10], P[MAX_N][11], prof[1<<11];

int main() {
    cin >> N;
    rep(i, 0, N) rep(j, 0, 10) cin >> F[i][j];
    rep(i, 0, N) rep(j, 0, 11) cin >> P[i][j];

    // bit全探索
    for (int i=0; i < (1<<10); i++) {
        prof[i] = 0;
        vector<int> cnt(N, 0);
        for (int j=0; j < 10; j++) {
            if ((i>>j) & 1) {  // 時間帯jに営業しているとき
                for (int n=0; n < N; n++) {
                    if (F[n][j]) cnt[n]++; // 時間帯jに店nが営業しているか
                }
            }
        }
        for (int n=0; n < N; n++) {
            prof[i] += P[n][cnt[n]];
        }
    }

    ll ans = prof[1];
    rep(i, 1, (1<<10)) chmax(ans, prof[i]);
    cout << ans << endl;
}
