// D - Coloring Dominoes
// ----------------------
// 問題
// https://atcoder.jp/contests/arc081/tasks/arc081_b

// 無理だ
// ----------------------

// 再帰で全探索
// color:
//     赤: 1
//     水: 2
//     緑: 3

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int MOD = 1000000007;

// solve
map<char, int> toInt;
vector<set<int>> G;

ll dfs(vector<int> color, char cur) {
    // 隣接する色を取得
    set<int> dc;  // duplicate color
    for (auto a : G[cur]) dc.insert(color[a]);

    // すでに3色あった場合
    if (dc.find(1)!=dc.end() && dc.find(2)!=dc.end() && dc.find(3)!=dc.end()) {
        return 0;
    }

    ll ans = 1;
    for (int i=1; i<=3; i++) {
        if (dc.find(i) == dc.end()) {
            color[cur] = i;
            ans *= dfs(color, cur+1);
            ans %= MOD;
            color[cur] = 0;
        }
    }

    return ans;
}

int main() {
    int N; cin >> N;
    string s1, s2; cin >> s1 >> s2;

    // 英字を数字に変換
    int now=0;
    rep(i, N) {
        if (!toInt[s1[i]]) toInt[s1[i]] = now++;
        if (!toInt[s2[i]]) toInt[s2[i]] = now++;
    }

    // 隣接リストに変換
    G.assign(now, set<int>());
    rep(i, N) {
        int upper=toInt[s1[i]], lower=toInt[s2[i]];
        if (i > 0) { // 左側を追加
            if (toInt[s1[i-1]] != upper) G[upper].insert(toInt[s1[i-1]]);
            if (toInt[s2[i-1]] != lower) G[lower].insert(toInt[s2[i-1]]);
        }
        if (i+1 < N) { // 右側を追加
            if (toInt[s1[i+1]] != upper) G[upper].insert(toInt[s1[i+1]]);
            if (toInt[s2[i+1]] != lower) G[lower].insert(toInt[s2[i+1]]);
        }
        // 上下を追加
        if (upper != lower) {
            G[upper].insert(lower);
            G[lower].insert(upper);
        }
    }

    dfs()
}
