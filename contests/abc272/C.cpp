#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    int N; cin >> N;

    // 偶数、奇数に分割
    vector<ll> odd, even;
    for (int i = 0; i < N; i++) {
        ll a; cin >> a;
        if (a & 1) {
            odd.push_back(a);
        } else {
            even.push_back(a);
        }
    }

    // ソート
    sort(ALL(odd), greater<ll>());
    sort(ALL(even), greater<ll>());

    ll ans = -1;

    // 奇数
    if (odd.size() >= 2) {
        chmax(ans, odd[0] + odd[1]);
    }

    // 偶数
    if (even.size() >= 2) {
        chmax(ans, even[0] + even[1]);
    }

    cout << ans << endl;
}