//       D - Restricted Permutation
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc223/tasks/abc223_d
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

vector<int> topological_sort(vector<vector<int>> G) {
    int N = G.size();
    vector<int> in_cnt(N, 0);
    for (auto edges : G) for (int to : edges) in_cnt[to]++;

    vector<int> ans;
    priority_queue<int, vector<int>, greater<int>> heap;
    for (int i = 0; i < N; i++) if (in_cnt[i]==0) heap.push(i);

    while (!heap.empty()) {
        int u = heap.top(); heap.pop();
        ans.push_back(u);
        for (auto v : G[u]) {
            in_cnt[v]--;
            if (in_cnt[v] == 0) {
                heap.push(v);
            }
        }
    }
    return ans;
}

int main() {
    int N, M; cin >> N >> M;
    vector<vector<int>> G(N);
    for (int i=0; i < M; i++) {
        int a, b; cin >> a >> b;
        G[a-1].push_back(b-1);
    }
    vector<int> ans = topological_sort(G);
    if (ans.size() != N) {
        cout << -1 << endl;
        return 0;
    }
    for (int v : ans) {
        cout << v+1 << endl;
    }
}
