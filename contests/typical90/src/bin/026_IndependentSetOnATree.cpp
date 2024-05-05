//   026 - Independent Set on a Tree（★4）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_z

// C++、エラー多すぎて嫌になるけどはやすぎる

// AC
// ----------------------------------------

// 解説
// https://kaage.hatenablog.com/entry/2021/05/04/144829

// 2部グラフという概念が必要らしい
// | 変で直接結ばれた頂点同士が互いに違う色になるように塗ることができるグラフ

// 木は必ず2部グラフになるから、depthが大きい順に出力すれば良い


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

vector<int> color;  // 深さ % 2、-1の場合は探索していない

void dfs(vector<vector<int>> &G, int now_point, int now_depth) {
    // 探索済みの場合
    if (color[now_point] != -1) return;

    // 探索済みでない場合
    color[now_point] = now_depth;
    for (auto next_point : G[now_point]) {
        dfs(G, next_point, (now_depth + 1) % 2);
    }
}

int main() {
    int N; cin >> N;  // 頂点の数
    vector<vector<int>> G(N);  // グラフ

    color.assign(N, -1);

    for (int i = 0; i < N-1; i++) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    dfs(G, 0, 0);

    // 色分けのうち多い方のグループ
    bool is_more = accumulate(color.begin(), color.end(), 0) > N / 2;

    int cnt = 0;
    for (int i = 0; i < N; i++) {
        if (cnt >= N / 2) break;
        if (color[i] == is_more) {
            cout << i+1 << " ";
            cnt++;
        }
    }
    cout << endl;
}