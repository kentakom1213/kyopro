//       Q1-7. 区間スケジューリング問題
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/363

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; cin >> N;
    vector<pair<int, int>> schedules(N);
    for (auto& [s, t] : schedules) cin >> s >> t;

    // 終了順でソート
    sort(schedules.begin(), schedules.end(), [](auto &l, auto &r) {
        return l.second < r.second;
    });

    // 貪欲にとる
    int res = 0, end = 0;
    for (int i = 0; i < N; i++) {
        if (end <= schedules[i].first) {
            res++;
            end = schedules[i].second;
        }
    }
    cout << res << endl;
}