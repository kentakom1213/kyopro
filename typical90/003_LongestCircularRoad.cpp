//     003 - Longest Circular Road（★4）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_c

// 後でやる
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    vector<set<int>> G(N);
    for (int i = 0; i < N; i++) {
        int a, b; cin >> a >> b;
        a--, b--;
        G[a].insert(b);
        G[b].insert(a);
    }


}