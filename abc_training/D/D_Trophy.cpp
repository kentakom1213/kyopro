//                D - Trophy               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc258/tasks/abc258_d
// ----------------------------------------

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
constexpr ll INF = numeric_limits<long long>::max();

int main() {
    int N, X; cin >> N >> X;
    vector<ll> A(N), B(N);
    rep(i, 0, N) cin >> A[i] >> B[i];

    ll sum=0, ans=INF;
    rep(i, 0, N) {
        if (i+1 > X) break;
        sum += A[i] + B[i];

        ll tmp = sum + (ll)B[i] * (X-i-1);
        chmin(ans, tmp);
    }

    cout << ans << endl;
}
