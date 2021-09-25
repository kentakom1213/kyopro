//     Q1-6. 巡回セールスマン問題 (貪欲法)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/365

// AC
// ----------------------------------------


#include <bits/stdc++.h>
using namespace std;

auto sq = [](int x) { return x * x; };
const float INF = 1 << 29;

int main() {
    int N; cin >> N;
    vector<pair<int, int>> points(N);
    for (auto& [x, y] : points) cin >> x >> y;
    vector<bool> is_visited(N, false);
    float res = 0;

    // 初期化
    int now_point = 0;
    is_visited[0] = true;

    for (int _ = 0; _ < N-1; _++) {

        float shortest = INF;
        auto [now_x, now_y] = points[now_point];
        // printf("now: (%d, %d)\n", now_x, now_y);

        for (int i = 0; i < N; i++) {
            if (is_visited[i]) continue;  // 訪れたところはパス

            auto [x, y] = points[i];
            float distance = sqrt( sq(now_x - x) + sq(now_y - y) );
            // printf("|(%d, %d), (%d, %d)| = %f\n", now_x, now_y, x, y, distance);
            if (distance < shortest) {
                shortest = distance;
                now_point = i;
                // cout << "next: " << now_point << endl;
            }
        }
        is_visited[now_point] = true;
        res += shortest;

        // print_vector(is_visited);
    }
    auto [now_x, now_y] = points[now_point];
    auto [origin_x, origin_y] = points[0];
    res += sqrt( sq(now_x - origin_x) + sq(now_y - origin_y) );
    cout << res << endl;
}