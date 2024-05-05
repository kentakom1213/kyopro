//          A - LR Constraints
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc124/tasks/arc124_a
// ----------------------------------------

// O(N+K) に

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
constexpr ll MOD = 998244353;

int main() {
    int N, K; cin >> N >> K;
    vector<ll> imos(N, 0), filled(N, 0);

    rep(i, K) {
        char c; int k; cin >> c >> k;
        k--;
        filled[k] = 1;
        if (c == 'L') {
            if (k+1 < N) imos[k+1]++;
        } else {
            imos[0]++;
            if (k+1 < N) imos[k+1]--;
        }
    }

    ll sum = 0;
    ll ans = 1LL;
    rep(i, N) {
        sum += imos[i];
        if (!filled[i]) ans = ans * sum % MOD;
    }
    cout << ans << endl;
}
