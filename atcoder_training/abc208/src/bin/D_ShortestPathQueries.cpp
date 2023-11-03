//       D - Shortest Path Queries 2
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc208/tasks/abc208_d
// ----------------------------------------

// N <= 400
// ワーシャルフロイド法で実装可能
// O(N^3)

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
constexpr int INF = 1<<27;

template <typename T>
void print_array(vector<vector<T>>& array) {
    int H = array.size();

    cerr << "{" << endl;
    for (int i = 0; i < H; i++) {
        int W = array.at(i).size();
        cerr << "  {";
        for (int j = 0; j < W; j++) {
            if (j < W - 1) cerr << array.at(i).at(j) << ", ";
            else cerr << array.at(i).at(j);
        }
        cerr << "}," << endl;
    }
    cerr << "}" << endl;
}

int main() {
    int N, M; cin >> N >> M;
    initArray(G, N, N, INF);
    rep(i, M) {
        int a, b, c; cin >> a >> b >> c;
        a--, b--;
        G[a][b] = c;
    }

    initArray(dist, N, N, INF);
}