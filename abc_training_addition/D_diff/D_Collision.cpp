//              D - Collision
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc209/tasks/abc209_d

// AC
// ----------------------------------------

// 頂点N個、辺N-1本である => 木
// これを利用する

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;

Array G;
vector<int> depth;  // centerからの距離
int N;

// 深さを返す
void dfs(int v) {
    for (int next : G[v]) {
        if (depth[next] != -1) continue;
        depth[next] = depth[v] + 1;
        dfs(next);
    }
}

int main() {
    int Q; cin >> N >> Q;
    G.assign(N, vector<int>());
    depth.assign(N, -1);

    vector<int> nodes(N);
    rep(i, N-1) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
        nodes[a]++;
        nodes[b]++;
    }

    int center = max_element(ALL(nodes)) - nodes.begin();
    depth[center] = 0;
    dfs(center);

    // クエリ処理
    rep(i, Q) {
        int c, d; cin >> c >> d;
        c--, d--;
        int dist = depth[c] + depth[d];
        cout << (dist % 2 == 0 ? "Town" : "Road") << endl;
    }

}