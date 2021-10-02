//             Q3-5. 頂点を塗る
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/414

// TLE
// ----------------------------------------

// TLEの計算量は O(n^2)
// N <= 10^5 であるから この計算量では間に合わない

// TLE
// #include <bits/stdc++.h>
// using namespace std;
// template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
// template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

// const int INF = 1 << 29;

// int main() {
//     int N, M; cin >> N >> M;
//     vector<vector<int>> G(N);
//     for (int i = 0; i < M; i++) {
//         int a, b; cin >> a >> b;
//         G[a].push_back(b);
//         G[b].push_back(a);
//     }
    
//     vector<int> is_visited(N, INF);
//     is_visited[0] = 0;
//     // 辺の更新 O(n^2)
//     for (int i = 0; i < N; i++) {
//         if (is_visited[i] == INF) continue;
//         for (auto next_point : G[i]) {
//             // printf("%d := min(%d, %d)\n", next_point, is_visited[next_point], is_visited[i] + 1);
//             chmin(is_visited[next_point], is_visited[i] + 1);
//         }
//     }

//     for (int k = 0; k < N; k++) {
//         for (int i = 0; i < N; i++) {
//             if (is_visited[i] == k) cout << i << " ";
//         }
//         cout << endl;
//     }
// }


// 計算量改善
#include <bits/stdc++.h>
using namespace std;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

const int INF = 1 << 29;

int main() {
    int N, M; cin >> N >> M;
    vector<vector<int>> G(N);
    for (int i = 0; i < M; i++) {
        int a, b; cin >> a >> b;
        G[a].push_back(b);
        G[b].push_back(a);
    }
    
    vector<int> is_visited(N, INF);
    is_visited[0] = 0;
    // 辺の更新 
    for (int i = 0; i < N; i++) {
        if (is_visited[i] == INF) continue;
        for (auto next_point : G[i]) {
            // printf("%d := min(%d, %d)\n", next_point, is_visited[next_point], is_visited[i] + 1);
            chmin(is_visited[next_point], is_visited[i] + 1);
        }
    }

    for (int k = 0; k < N; k++) {
        for (int i = 0; i < N; i++) {
            if (is_visited[i] == k) cout << i << " ";
        }
        cout << endl;
    }
}