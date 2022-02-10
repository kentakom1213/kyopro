#include <bits/stdc++.h>
using namespace std;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

constexpr int LOOP_COUNT = 10000000;
constexpr int FIELD_MIN = 0;
constexpr int FIELD_MAX = 10000;

struct add {
    int lbrt[4];  // left bottom right top

    void print() {
        printf("%d %d %d %d\n", lbrt[0], lbrt[1], lbrt[2], lbrt[3]);
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
        // ランダムに選ぶ
        int choose = randint(n);
        auto& [left, bottom, right, top] = adds[choose].lbrt;

        // 拡大する辺
        int ext = randint(4);
        switch (ext) {
            break; case 0:
                left--;
                chmax(left, FIELD_MIN);
            break; case 1:
                bottom--;
                chmax(bottom, FIELD_MIN);
            break; case 2:
                right++;
                chmin(right, FIELD_MAX);
            break; case 3:
                top++;
                chmin(top, FIELD_MAX);
        }

        // 衝突判定
        bool is_in_collision = false;
        for (int i=0; i<n; i++) if (i != choose) {
            is_in_collision |= adds[i].is_in_collision(left, bottom, right, top);
        }

        // 衝突してたら元に戻す
        if (is_in_collision) {
            switch (ext) {
                break; case 0:
                    left++;
                break; case 1:
                    bottom++;
                break; case 2:
                    right--;
                break; case 3:
                    top--;
            }
        }
    }

    // 表示
    for (int i=0; i<n; i++) {
        adds[i].print();
    }
}
