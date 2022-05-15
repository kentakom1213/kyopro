
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

ll combi(int n, int r) {
    if (r == 0) return 1;
    if (n < r+r) return combi(n, n-r);
    return (n-r+1) * combi(n, r-1) / r;
}

int main() {
    string N; cin >> N;
    int K; cin >> K;
    int d = N.size();

    ll ans = 0;
    for (int i = K; i < d; i++) {
        ans += powl(9, K) * combi(i-1, K-1);
    }

    for (int i = 0; i < d; i++) {
        
    }
}
