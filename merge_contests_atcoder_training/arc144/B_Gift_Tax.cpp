//               B - Gift Tax              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc144/tasks/arc144_b
// ----------------------------------------

// 解の存在を考えて二分探索

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
template <typename T, typename U> inline T div_ceil(T x, U y) {return (x + y - 1) / y;}
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    ll N, a, b;
    cin >> N >> a >> b;
    vector<ll> A(N);
    rep(i, 0, N) cin >> A[i];

    auto isOK = [&](ll x) -> bool {
        ll cnt = 0;
        for (ll v : A) {
            if (v < x) { // vをx以上にするために何回aを足す必要があるか
                cnt -= div_ceil(x-v, a);
            } else if (v > x) {  // 何回までならbを引いてもx以上であるか
                cnt += (v-x) / b;
            }
        }
        return cnt >= 0;
    };

    ll ok=1, ng=1001001001;
    while (ng - ok > 1) {
        ll mid = (ok + ng) / 2;
        if (isOK(mid)) ok = mid;
        else ng = mid;
    }

    cout << ok << endl;
}
