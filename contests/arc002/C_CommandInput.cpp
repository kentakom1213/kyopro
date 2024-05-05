//            C - コマンド入力
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc002/tasks/arc002_3
// ----------------------------------------

// コマンドの選び方
// 4 ^ 4 = 256
// 全探索すると、256000
// 十分間に合う

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int N; cin >> N;
    string com; cin >> com;
    string C = "ABXY";

    int ans = 1<<30;
    rep(l1, 4) rep(l2, 4) rep(r1, 4) rep(r2, 4) {
        int i=0, cnt=0;
        while (i < N) {
            if (i+1 == N) {
                cnt++;
                break;
            }
            // L, Rに置換できるか
            bool can_replace_L = com[i]==C[l1] && com[i+1]==C[l2];
            bool can_replace_R = com[i]==C[r1] && com[i+1]==C[r2];
            if (can_replace_L || can_replace_R) {
                cnt++;
                i += 2;
            } else {
                cnt++;
                i++;
            }
        }
        chmin(ans, cnt);
    }
    cout << ans << endl;
}
