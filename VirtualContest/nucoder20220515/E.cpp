
#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// x以下の最大のnの累乗
ll lb_x(ll x, ll n) {
    ll i = 1;
    while (i*n <= x) {
        i *= n;
    }
    return i;
}

ll dfs(ll x) {
    if (x == 0) return 0;

    ll mul_6 = lb_x(x, 6);
    ll mul_9 = lb_x(x, 9);

    ll ans = 1LL << 30;
    if (mul_6 > 1) {
        chmin(ans, dfs(x - mul_6) + 1);
    }
    if (mul_9 > 1) {
        chmin(ans, dfs(x - mul_9) + 1);
    }
    if (mul_6 == 1) {
        ans = x;
    }
    return ans;
}

int main() {
    ll N; cin >> N;
    cout << dfs(N) << endl;
}
