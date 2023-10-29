//     021 - Come Back in One Piece（★5）    
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_u
// ----------------------------------------

// 強連結成分分解（SCC）

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

vector<vector<int>> G, rG;
vector<bool> used;
vector<int> postorder;
ll cnts = 0;

void dfs(int cur) {
    used[cur] = true;
    for (int nxt : G[cur]) {
        if (used[nxt] == false) dfs(nxt);
    }
    postorder.push_back(cur);
}

void rdfs(int cur) {
    used[cur] = true;
    cnts++;
    for (int nxt : rG[cur]) {
        if (used[nxt] == false) rdfs(nxt);
    }
}

int main() {
    int N, M; cin >> N >> M;
    G.assign(N, {});
    rG.assign(N, {});  // rG: 向きを逆にしたグラフ
    for (int i = 0; i < M; i++) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        rG[b].push_back(a);
    }

    // 初期化
    used.assign(N, false);

    // 塗られていない頂点からdfs
    for (int i = 0; i < N; i++) {
        if (used[i] == false) dfs(i);
    }

    // 逆向きにdfs
    fill(ALL(used), false);  // used配列を再度falseで初期化
    reverse(ALL(postorder));  // 帰りがけ順を逆転

    ll ans = 0;
    for (int i : postorder) {
        if (used[i] == true) continue;
        cnts = 0;
        rdfs(i);
        ans += cnts * (cnts - 1LL) / 2LL;
    }

    cout << ans << endl;
}
