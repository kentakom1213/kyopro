//              D - Collision
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc209/tasks/abc209_d

// pass
// ----------------------------------------

// 愚直BFS (もちろんTLE)
// #include <bits/stdc++.h>
// using namespace std;
// #define rep(i, n) for (int i = 0; i < (int)(n); i++)
// #define ALL(A) A.begin(), A.end()
// #define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
// typedef long long ll;
// typedef pair<int, int> point;
// typedef vector<vector<int>> Array;

// Array G;
// int N;

// // 深さを返す
// int bfs(int s, int t) {
//     vector<bool> is_visited(N, false);
//     queue<pair<int, int>> q;
//     q.push(make_pair(s, 0));
//     while (!q.empty()) {
//         auto [now, depth] = q.front(); q.pop();

//         is_visited[now] = true;
//         // 探索
//         for (int next : G[now]) {
//             // printf("now:%d -> next:%d (%s)\n",now, next, (is_visited[next] ? "o" : "x"));
//             if (next == t) return depth + 1;
//             if (is_visited[next]) continue;
//             q.push(make_pair(next, depth+1));
//         }
//     }
//     return -1;
// }

// int main() {
//     int Q; cin >> N >> Q;
//     G.assign(N, vector<int>());
//     rep(i, N-1) {
//         int a, b; cin >> a >> b;
//         a--, b--;
//         G[a].push_back(b);
//         G[b].push_back(a);
//     }

//     vector<pair<int, int>> queries(Q);
//     rep(i, Q) {
//         int c, d; cin >> c >> d;
//         c--, d--;
//         queries[i] = make_pair(c, d);
//     }

//     for (auto [c, d] : queries) {
//         int depth = bfs(c, d);
//         cout << (depth % 2 == 0 ? "Town" : "Road") << endl;
//     }
// }