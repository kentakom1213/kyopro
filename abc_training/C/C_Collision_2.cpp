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
typedef pair<int, int> P;

int main() {
    int N; cin >> N;
    P points[N];
    for (auto &[x, y] : points) {
        scanf("%d %d", &x, &y);
    }
    string dir; cin >> dir;

    bool is_in_collision = false;

    // y: (x, dir) のmapを作成
    map<int, P> mp;
    rep(i, 0, N) {
        auto [x, y] = points[i];
        int d = dir[i] == 'R' ? 1 : 0;  // R->1, L->0
        if (mp.find(y) == mp.end()) {
            mp[y] = {x, d};
        } else {
            auto [xx, dd] = mp[y];
            // 方向が同じとき
            if (dd == d) {
                if (d) mp[y] = {min(xx, x), d};
                else mp[y] = {max(xx, x), d};
            }
            // 方向が逆のとき
            else {
                if (d) is_in_collision |= dd <= d;
                else is_in_collision |= d <= dd;
            }
        }
    }

    printf("%s\n", is_in_collision ? "Yes" : "No");
}
