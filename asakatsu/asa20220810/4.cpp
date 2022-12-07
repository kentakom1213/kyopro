// https://atcoder.jp/contests/abc127/tasks/abc127_d

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N, M; cin >> N >> M;
    vector<ll> A(N);
    rep(i, 0, N) cin >> A[i];
    vector<pll> CB(M);
    for (auto &[c, b] : CB) cin >> b >> c;
    CB.push_back({0, 1010101});  // 埋めるよう

    // ソート
    sort(ALL(A));
    sort(ALL(CB), greater<pll>());

    // 貪欲に
    int now = 0;
    ll ans = 0;
    for (auto [c, b] : CB) {
        rep(i, 0, b) {
            ans += max(c, A[now]);
            now++;
            if (now == N) break;
        }
        if (now == N) break;
    }
    cout << ans << endl;
}