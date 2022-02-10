#include <bits/stdc++.h>
using namespace std;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

constexpr int LOOP_COUNT = 1000000;
constexpr int FIELD_MIN = 0;
constexpr int FIELD_MAX = 10000;
constexpr int INCREASE_RATE = 10000;

struct add {
    int lbrt[4];   // left bottom right top

    void print() {
        printf("%d %d %d %d\n", lbrt[0], lbrt[1], lbrt[2], lbrt[3]);
    }

    double area() {
        return (lbrt[2] - lbrt[0]) * (lbrt[3] - lbrt[1]);
    }

    bool is_in_collision(int left, int bottom, int right, int top) {
        bool ans = (max(lbrt[0], left) < min(lbrt[2], right)) && (max(lbrt[1], bottom) < min(lbrt[3], top));
        return ans;
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
        int choose = randint(n);
        auto& [left, bottom, right, top] = adds[choose].lbrt;
        auto [x, y, r] = req[choose];
        int reset[4] = {left, bottom, right, top};

        // 求められている面積との差
        int diff = (r - adds[choose].area()) / INCREASE_RATE;

        // 辺を拡大
        int ext = randint(4);
        switch (ext) {
            break; case 0:
                left -= diff;
                chmax(left, FIELD_MIN);
            break; case 1:
                bottom -= diff;
                chmax(bottom, FIELD_MIN);
            break; case 2:
                right += diff;
                chmin(right, FIELD_MAX);
            break; case 3:
                top += diff;
                chmin(top, FIELD_MAX);
        }

        // 面積が正になるように調整
        bool is_not_positive_area = adds[choose].area() < 0;

        // 衝突判定
        bool is_in_collision = false;
        for (int i=0; i<n; i++) if (i != choose) {
            is_in_collision = adds[i].is_in_collision(left, bottom, right, top);
            if (is_in_collision) break;
        }

        // 面積が負の場合 / 衝突している場合、元に戻す
        if (is_not_positive_area || is_in_collision) {
            left = reset[0];
            bottom = reset[1];
            right = reset[2];
            top = reset[3];
        }
    }

    // 表示
    for (int i=0; i<n; i++) {
        adds[i].print();
    }
}
