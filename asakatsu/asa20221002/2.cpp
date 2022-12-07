#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// コーナーケース
// oxxxxxxx

int main() {
    int h, w; cin >> h >> w;
    if (h == 1 || w == 1) cout << 1 << endl;
    else if (h*w & 1) cout << h*w/2+1 << endl;
    else cout << h*w/2 << endl;
}