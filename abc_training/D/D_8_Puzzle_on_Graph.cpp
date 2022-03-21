//         D - 8 Puzzle on Graph
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc224/tasks/abc224_d
// ----------------------------------------

/* comment
## 方針
- コマの状態を順列と考えて保存
- 最短経路問題に帰着する
- 9! = 362880 だから問題なし 
*/

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
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

#define state array<int, 9>

int M;
vector<vector<int>> G;
state P;

state OK = {0, 1, 2, 3, 4, 5, 6, 7, 8};

state swap_state(state s, int branc, int nxt) {
    swap(s[branc], s[nxt]);
    return s;
}

int main() {
    cin >> M;
    G.assign(9, vector<int>());

    rep(i, M) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    int rem = 45;
    rep(i, 8) {
        int p; cin >> p;
        rem -= p;
        P[i] = p-1;
    }
    P[8] = rem-1;

    // mapで管理
    map<state, int> is_visited;

    // bfs
    queue<pair<state, vec2>> q;
    q.push(make_pair(P, make_pair(8, 0)));
    while (!q.empty()) {
        auto [cur, data] = q.front(); q.pop();
        auto [branc, dist] = data;

        // 記録
        is_visited[cur] = 1;

        if (cur == OK) {
            cout << dist << endl;
            return 0;
        }

        for (int nxt : G[branc]) {
            state nxt_state = swap_state(cur, branc, nxt);
            if (is_visited[nxt_state]) continue;

            // キューに追加
            q.push(make_pair(nxt_state, make_pair(nxt, dist+1)));
        }
    }

    cout << -1 << endl;
}
