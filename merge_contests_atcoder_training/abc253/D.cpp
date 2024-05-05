
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

#define sum(n) (n * (n+1) / 2)

int main() {
    ll N, A, B;
    cin >> N >> A >> B;

    ll ans = sum(N);

    if (A == B) {
        ans -= A * sum(N / A);
    }
    else {
        ans -= A * sum(N / A);
        ans -= B * sum(N / B);
        ans += A * B * sum(N / (A*B));
    }

    cout << ans << endl;
}
