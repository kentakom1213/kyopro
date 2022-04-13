//             A - Airport Bus             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/agc011/tasks/agc011_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
const ll INF = 1LL << 50;

const int MAX_SIZE = 101010;
ll N, C, K, T[MAX_SIZE];

int main() {
    cin >> N >> C >> K;
    rep(i, N) cin >> T[i];
    sort(T, T+N);

    ll ans=0, now=-INF, cnt=0;
    rep(i, N) {
        ll t = T[i];
        if (t <= now+K && cnt < C) {
            cnt++;  // 乗車させる
        } else {
            ans++;  // 新しいバスを手配
            now = t;
            cnt = 1;
        }
    }

    cout << ans << endl;
}
