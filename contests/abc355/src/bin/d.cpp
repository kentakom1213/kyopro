#include <bits/stdc++.h>
// #include <atcoder/all>
using namespace std;
// using namespace atcoder;
#define rep(i, a, n) for(int i = a; i < n; i++)
#define rrep(i, a, n) for(int i = a; i >= n; i--)
#define inr(l, x, r) (l <= x && x < r)
#define ll long long
#define ld long double

// using mint = modint1000000007;
// using mint = modint998244353;
constexpr int IINF = 1001001001;
constexpr ll INF = 9e18;

template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main(){
    int N;
    cin >> N;
    vector<pair<int, int>> LR;
    rep(i, 0, N) {
        int l, r; cin >> l >> r;
        LR.push_back({l, 1});
        LR.push_back({r + 1, -1});
    }

    sort(LR.begin(), LR.end());

    ll tmp = 0;
    ll ans = 0;

    for (auto [x, y] : LR) {
        tmp += y;

        if (y == -1) {
            ans += tmp;
        }
    }

    cout << ans << endl;
    
    return 0;
}
