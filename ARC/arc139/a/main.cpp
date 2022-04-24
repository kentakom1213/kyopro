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

int ctz(ll x) {
    int ans = 0;
    while (x % 2 == 0) {
        ans++;
        x /= 2;
    }
    return ans;
}

// xより真に大きい最小のkの奇数倍を返す
ll min_mul(ll x, ll k) {
    return (x + k) / k * k;
}

int main() {
    ll N; cin >> N;
    vector<ll> T(N), ans(N);
    rep(i, 0, N) cin >> T[i];

    // 貪欲に
    ans[0] = 1LL << T[0];
    rep(i, 0, N-1) {
        ll mul = min_mul(ans[i], 1LL<<T[i+1]);
        if (ctz(mul) > T[i+1]) mul += 1LL << T[i+1];
        ans[i+1] = mul;
    }

    cout << ans[N-1] << endl;
}