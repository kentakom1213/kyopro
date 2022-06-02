// https://atcoder.jp/contests/abc112/tasks/abc112_d

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

// 約数列挙
set<int> factors(int n) {
    set<int> res;
    for (int i=1; i*i<=n; i++) {
        if (n%i==0) {
            res.insert(i);
            res.insert(n/i);
        }
    }
    return res;
}

int main() {
    ll N, M; cin >> N >> M;
    set<int> fs = factors(M);
    int ans = 0;
    for (int f : fs) {
        if (M/N >= f) chmax(ans, f);
    }
    cout << ans << endl;
}
