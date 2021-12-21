//        C - Monsters Battle Royale
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc118/tasks/abc118_c
// ----------------------------------------

// segment fault

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    ll N; cin >> N;
    set<ll> vals;
    for (int i = 0; i < N; i++) {
        ll val; cin >> val;
        vals.insert(val);
    }

    // 最小の値で貪欲に剰余をとっていく、0となった値は除く
    ll mini;
    do {
        // print
        for (auto itr = vals.begin(); itr != vals.end(); itr++) {
            cerr << *itr << " " << endl;
        }
        cerr << endl;

        mini = *vals.begin();
        vals.erase(vals.begin());
        for (auto itr = vals.begin(); itr != vals.end(); itr++) {
            vals.insert(*itr % mini);
            vals.erase(itr);
        }
    } while (!vals.empty());

    cout << mini << endl;
}