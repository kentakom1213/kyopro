// https://atcoder.jp/contests/arc039/tasks/arc039_b

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// --- combination ---
// https://drken1215.hatenablog.com/entry/2018/06/08/210000
const int MAX_COMB = 707070;
ll fact[MAX_COMB], finv[MAX_COMB], inv[MAX_COMB];

void INIT_COMB_ARRAY(int p) {
    fact[0] = fact[1] = 1;
    finv[0] = finv[1] = 1;
    inv[1] = 1;
    for (int i=2; i < MAX_COMB; i++) {
        fact[i] = fact[i-1] * i % p;
        inv[i] = p - inv[p%i] * (p / i) % p;
        finv[i] = finv[i-1] * inv[i] % p;
    }
}

ll comb(ll n, ll r, int p) {
    if (n < r) return 0;
    if (n < 0 || r < 0) return 0;
    return fact[n] * (finv[r] * finv[n-r] % p) % p;
}
// -------------------

int main() {
    INIT_COMB_ARRAY(MOD);
    
    ll n, k; cin >> n >> k;
    if (n > k) {
        cout << comb(n+k-1, k, MOD) << endl;  // 重複組合せ
    }
    else {
        cout << comb(n, k%n, MOD) << endl;  // あまりを分配する組合せ
    }
}