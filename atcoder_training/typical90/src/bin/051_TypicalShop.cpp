//         051 - Typical Shop（★5）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ay
// ----------------------------------------

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


int main() {
    ll N, K, P; cin >> N >> K >> P;
    int la=N/2, lb=N-N/2;
    vector<ll> A(la), B(lb);
    rep(i, la) cin >> A[i];
    rep(i, lb) cin >> B[i];

    vector<vector<ll>> p_sum_A(la+1), p_sum_B(lb+1);

    // Aの部分和を全列挙
    rep(i, 1<<la) {
        int cnt = 0;
        ll p_sum_a = 0;
        rep(j, la) {
            if ((i>>j) & 1) {
                cnt++;
                p_sum_a += A[j];
            }
        }
        p_sum_A[cnt].push_back(p_sum_a);
    }
    rep(c, la+1) sort(ALL(p_sum_A[c]));
    
    // Bの部分和を全列挙
    rep(i, 1<<lb) {
        int cnt = 0;
        ll p_sum_b = 0;
        rep(j, lb) {
            if ((i>>j) & 1) {
                cnt++;
                p_sum_b += B[j];
            }
        }
        p_sum_B[cnt].push_back(p_sum_b);
    }
    rep(c, lb+1) sort(ALL(p_sum_B[c]));

    // K個の和がPを超えないように和をとっていく
    ll ans = 0;
    rep(i, K+1) {
        if (i > la) break;
        for (ll p_sum_a : p_sum_A[i]) {
            ll remK = K - i;
            ll remP = P - p_sum_a + 1;
            if (remK <= lb) {
                auto ptr = lower_bound(ALL(p_sum_B[remK]), remP);
                ll cnt = ptr - p_sum_B[remK].begin();
                ans += cnt;
            }
        }
    }

    cout << ans << endl;
}
