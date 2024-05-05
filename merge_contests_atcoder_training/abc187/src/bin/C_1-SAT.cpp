//                C - 1-SAT
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc187/tasks/abc187_c

// 実装がめんどくさかった
// 解説を見ると、ただ s と '!' + s が存在すればいいかを調べればよかったっぽい

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N;
    cin >> N;
    map<string, pair<bool, bool>> strs;
    for (int i = 0; i < N; i++) {
        string s;
        cin >> s;
        int excl = 0;
        if (s[0] == '!') {
            s = s.substr(1);
            excl = 1;
        }

        if (excl) strs[s].second = true;
        else strs[s].first = true;

    }

    bool is_find = false;
    for (const auto& [k, v] : strs) {
        if (v.first && v.second) {
            cout << k << endl;
            is_find = true;
            break;
        }
    }
    if (not is_find) cout << "satisfiable" << endl;
}
