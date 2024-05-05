//           D - Preparing Boxes           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc134/tasks/abc134_d
// ----------------------------------------

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

int N, ans[202020];
bool A[202020];

int main() {
    cin >> N;
    rep(i, 1, N+1) cin >> A[i];

    for (int i=N; i>=1; i--) {
        ll sum = 0;
        for (int j=i; j<=N; j+=i) sum += ans[j];
        if ((sum&1) ^ A[i]) {
            ans[i] = 1;
        }
    }

    vector<ll> out;
    rep(i, 1, N+1) if (ans[i]) out.push_back(i);

    cout << out.size() << endl;
    for (ll a : out) cout << a << " "; cout << endl;
}
