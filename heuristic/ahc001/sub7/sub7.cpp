#include <bits/stdc++.h>
using namespace std;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
#define sq(n) (n)*(n)

constexpr int LOOP_COUNT    = 10000000;
constexpr int FIELD_MIN     = 0;
constexpr int FIELD_MAX     = 10000;
constexpr int INCREASE_RATE = 10000;
constexpr int MOVE_RATE     = 1000;
constexpr int DECREASE_RATE = 5;
constexpr int DECREASE_DIFF = 10;

struct add {
    int lbrt[4];   // left bottom right top

    void print() {
        printf("%d %d %d %d\n", lbrt[0], lbrt[1], lbrt[2], lbrt[3]);
    }

    double area() {
        return (lbrt[2] - lbrt[0]) * (lbrt[3] - lbrt[1]);
    }

    double score(int x, int y, int r) {
        return 1 - sq(1 - min((double)r, area())/max((double)r, area()));
    }

    bool is_contain(int x, int y) {
        return (lbrt[0] <= x && x < lbrt[2]) && (lbrt[1] <= y && y < lbrt[3]);
    }

    bool is_in_collision(int left, int bottom, int right, int top) {
        bool ans = (max(lbrt[0], left) < min(lbrt[2], right)) && (max(lbrt[1], bottom) < min(lbrt[3], top));
        return ans;
    }

    void expand(int dir, int diff) {
        if (dir <= 1) {
            lbrt[dir] -= diff;
            chmax(lbrt[dir], FIELD_MIN);
        } else {
            lbrt[dir] += diff;
            chmin(lbrt[dir], FIELD_MAX);
        }
    }

    void move(int dir, int diff) {
        auto& [left, bottom, right, top] = lbrt;
        switch (dir) {
            break; case 0:  // 左に移動
                chmin(diff, left - FIELD_MIN);
                left -= diff;
            break; case 1:  // 下に移動
                chmin(diff, bottom - FIELD_MIN);
                bottom -= diff;
            break; case 2:  // 右に移動
                chmin(diff, FIELD_MAX - right);
                right += diff;
            break; case 3:  // 上に移動
                chmin(diff, FIELD_MAX - top);
                top += diff;
        }
    } 

    void reset(int prev[]) {
        for (int i=0; i<4; i++) {
            lbrt[i] = prev[i];
        }
    }
};

int randint(int rand_max) {
    return rand() % rand_max;
}

int main() {
    srand(time(NULL));

    int n; cin >> n;
    vector<tuple<int, int, int>> req(n);
    for (auto& [x, y, r] : req) {
        cin >> x >> y >> r;
    }

    // 初期化
    vector<add> adds(n);
    for (int i=0; i < n; i++) {
        auto [x, y, r] = req[i];
        auto& [left, bottom, right, top] = adds[i].lbrt;
        left = x;
        bottom = y;
        right = x+1;
        top = y+1;
    }

    for (int i=0; i < LOOP_COUNT; i++) {

        //// 面積を変更する
        int choose = randint(n);
        auto& [left, bottom, right, top] = adds[choose].lbrt;
        auto [x, y, r] = req[choose];
        int reset[4] = {left, bottom, right, top};
        double before_score = adds[choose].score(x, y, r);

        // 求められている面積との差
        int diff = (r - adds[choose].area()) / INCREASE_RATE;

        // 辺を拡大
        int ext = randint(4);
        int increase = randint(DECREASE_RATE);
        if (increase || diff > 0) {
            adds[choose].expand(ext, diff);
        } else {
            adds[choose].expand(ext, -DECREASE_DIFF);
        }

        // ランダムに移動させる
        int dir = randint(4);
        int move_diff = adds[choose].area() / MOVE_RATE;
        adds[choose].move(dir, move_diff);

        // 面積が正になるように調整
        bool is_not_positive_area = adds[choose].area() < 0;

        // 衝突判定
        bool is_in_collision = false;
        for (int i=0; i<n; i++) if (i != choose) {
            is_in_collision = adds[i].is_in_collision(left, bottom, right, top);
            if (is_in_collision) break;
        }

        // 中心を含んでいるか判定
        bool is_not_contain_center = !adds[choose].is_contain(x, y);

        // 移動後のスコア
        double after_score = adds[choose].score(x, y, r);
        bool is_decrease_score = before_score > after_score;

        // 条件を満たさない場合、元に戻す
        if (is_decrease_score || is_not_contain_center || is_not_positive_area || is_in_collision) {
            adds[choose].reset(reset);
        }
    }

    // 表示
    for (int i=0; i<n; i++) {
        adds[i].print();
    }
}
