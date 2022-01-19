//         016 - Minimum Coins（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_p

// AC (解説)
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    ll N; cin >> N;
    ll A, B, C; cin >> A >> B >> C;
    int MAX = 10000;

    int ans = MAX;
    rep(x, MAX) {
        rep(y, MAX) {
            ll Cz = N - A*x - B*y;
            if (Cz % C != 0 || Cz < 0) continue;
            int z = Cz / C;
            ans = min(ans, x + y + z);
        }
    }
    cout << ans << endl;
}
