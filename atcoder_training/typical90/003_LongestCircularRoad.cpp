//     003 - Longest Circular Road（★4）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_c

// 参考
// https://qiita.com/nomikura/items/a4c5be6c72ce854d7ce4

// AC
// ----------------------------------------

// グラフの直径をもとめる
// メモ化再帰

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
#define initGraph(name, n) vector<vector<int>> name(n, vector<int>());
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

constexpr int maxV = 111111;
Array tree;

void dfs(int from, int curr, int dist, int &maxDist, int &maxVertex) {
    // 距離と終点を更新
    if (dist > maxDist) {
        maxDist = dist;
        maxVertex = curr;
    }

    for (auto to : tree[curr]) {
        // 逆流を防ぐ
        if (to == from) continue;
        dfs(curr, to, dist+1, maxDist, maxVertex);
    }
}

int main() {
    int N; cin >> N;
    tree.assign(N, vector<int>());
    rep(i, N-1) {
        int a, b; cin >> a >> b;
        a--, b--;
        tree[a].push_back(b);
        tree[b].push_back(a);
    }

    // 木の直径を求める
    int start = 0, end = 0, maxDist = 0;
    dfs(-1, start, 0, maxDist, end);

    start = end, end = 0, maxDist = 0;
    dfs(-1, start, 0, maxDist, end);

    cout << maxDist + 1 << endl;
}