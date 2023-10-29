//            G - Longest Path
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_g

// 配列を引数に渡す時は絶対に&つけよう

// AC
// ----------------------------------------

// 順番がわからない
// こういうときはメモ化再帰

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
#define initGraph(name, n) vector<vector<int>> name(n, vector<int>());
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }


vector<int> dp;

int f(const Array &G, int x) {
    if (dp[x] != -1) return dp[x];
    int fans = 0;
    for (auto next : G[x]) {
        chmax(fans, f(G, next) + 1);  // dfs
    }
    return dp[x] = fans;
}

int main() {
    int N, M; cin >> N >> M;
    dp.assign(N, -1);
    initGraph(G, N);
    rep(i, M) {
        int x, y; cin >> x >> y;
        x--, y--;
        G[x].push_back(y);
    }
    
    int res = 0;
    rep(i, N) {
        chmax(res, f(G, i));
    }

    cout << res << endl;
}
