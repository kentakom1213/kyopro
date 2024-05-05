//             C - Collision 2             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc243/tasks/abc243_c
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
constexpr int INF = 1 << 30;
using pll = pair<ll, ll>;

int main() {
    int N; cin >> N;
    pll points[N];
    for (auto &[x, y] : points) {
        scanf("%lld %lld", &x, &y);
    }
    string dir; cin >> dir;

    // yで分類
    map<ll, pll> mp;  // {y: [L, R]}
    rep(i, 0, N) {
        auto [x, y] = points[i];
        if (mp.find(y) == mp.end()) mp[y] = {-INF, INF};
        if (dir[i] == 'L') {
            mp[y].first = max(x, mp[y].first);
        } else {
            mp[y].second = min(x, mp[y].second);
        }
    }

    for (auto [_, p] : mp) {
        auto [l, r] = p;
        if (r <= l) {cout << "Yes" << endl; return 0;}
    }
    cout << "No" << endl;
}
