// C - Bridge
// -----------
// 問題
// https://atcoder.jp/contests/abc075/tasks/abc075_c

// AC
// -----------

// N, Mの値が小さいため、すべての辺に関してDFSをしても間に合いそう

// 辺の列挙: O(N^2)
// dfs一回: O(N+M)
// -> O(N^2(N+M))

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<long long>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

void dfs(int cur, Array &G, vector<int> &is_visited) {
    is_visited[cur] = 1;
    for (auto nxt : G[cur]) {
        if (is_visited[nxt]) continue;
        dfs(nxt, G, is_visited);
    }
}

int main() {
    int N, M; cin >> N >> M;
    Array G(N);
    vector<vec2> edges(M);
    rep(i, M) {
        int a, b; cin >> a >> b;
        a--, b--;
        edges[i] = make_pair(a, b);
    }

    // 全ての辺について調べる
    int cnt = 0;
    rep(i, M) {
        // edges[i]が存在しないグラフを生成
        Array G(N);
        rep(j, M) {
            if (i == j) continue;
            auto [a, b] = edges[j];
            G[a].push_back(b);
            G[b].push_back(a);
        }

        // dfsで連結判定
        vector<int> is_visited(N, 0);
        dfs(0, G, is_visited);

        // 全て探索できなかったとき、edges[i]は橋である
        if (accumulate(ALL(is_visited), 0) < N) cnt++;
    }
    
    cout << cnt << endl;
}