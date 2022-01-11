
// H, W が小さいため、全探索
// 組み合わせは 20 * 20 / 2 = 200通り
// これらについてBFS

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()

typedef vector<vector<int>> Array;
typedef pair<int, int> point;

int H, W;
vector<string> field;
vector<point> direction = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

int bfs(point start, point goal) {
    queue<point> q;
    q.push(start);
    Array is_visited(H, vector<int>(W, -1));
    is_visited[start.first][start.second] = 0;
    // print_array(is_visited);

    while (!q.empty()) {
        auto [cy, cx] = q.front();  // current (x, y)
        q.pop();
        // printf("now:(%d, %d)\n", cy, cx);

        if (cy == goal.first && cx == goal.second) {
            return is_visited[cy][cx];
        }

        for (auto [dy, dx] : direction) {
            int ny = cy + dy;  // 次の座標を確定
            int nx = cx + dx;

            // 座標が正しい場合はキューに追加
            if (0 <= ny && ny < H
                && 0 <= nx && nx < W
                && field[ny][nx] == '.'
                && is_visited[ny][nx] == -1) {
                // printf("next:(%d, %d)\n", ny, nx);
                is_visited[ny][nx] = is_visited[cy][cx] + 1;
                q.push(make_pair(ny, nx));
            }
        }

        // print_array(is_visited);
    }
}

int main() {
    cin >> H >> W;
    field.assign(H, "");
    rep(i, H) cin >> field[i];

    // point start = {2, 0};
    // point goal = {0, 4};
    // printf("start: (%d, %d), goal: (%d, %d)\n", start.first, start.second, goal.first, goal.second);

    int longest = 0;
    for (int i = 0; i < H*W; i++) {
        for (int j = i+1; j < H*W; j++) {
            point start = {i / W, i % W};
            point goal = {j / W, j % W};
            if (field[start.first][start.second] == '#'
                || field[goal.first][goal.second] == '#') continue;
            // printf("start: (%d, %d), goal: (%d, %d)\n", start.first, start.second, goal.first, goal.second);

            int shortest = bfs(start, goal);
            longest = max(longest, shortest);
        }
    }

    cout << longest << endl;
}