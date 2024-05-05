//            C - Scc Puzzle
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc069/tasks/arc069_a
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    ll N, M; cin >> N >> M;

    auto isOK = [&](ll x) {
        if (x <= N) return 2*x <= M;
        else return 2*(x-N) <= M && 2*x <= M-2*(x-N);
    };

    // xを2分探索
    ll l = 0, r = 10101010101010;
    while (r-l > 1) {
        ll mid = (l+r) / 2;
        if (isOK(mid)) l = mid;
        else r = mid;
    }

    cout << l << endl;
}
