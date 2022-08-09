//               B - Gift Tax              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc144/tasks/arc144_b
// ----------------------------------------

/*
## 方針
A <- 入力

while:
    x <- min(A)
    y <- max(A)
    if x+a > y-b:
        print x
        break

*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    ll N, a, b;
    cin >> N >> a >> b;
    set<ll> A;
    rep(i, 0, N) {
        ll a; cin >> a;
        A.insert(a);
    }

    // 貪欲に処理
    while (true) {
        ll x=*A.begin(), y=*A.end();
        A.erase(x);
        A.erase(y);
        
        if (x+a > y-b) {
            cout << max(x+a, y-b) << endl;
            return 0;
        }

        A.insert(x+a);
        A.insert(y-b);
    }
}
