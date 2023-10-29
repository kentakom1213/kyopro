//       027 - Sign Up Requests （★2）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_aa

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    map<string, int> users;
    vector<int> res;

    for (int i = 0; i < N; i++) {
        string s; cin >> s;
        if (users[s]) continue;
        else {
            res.push_back(i+1);
            users[s]++;
        }
    }

    for (auto n : res) cout << n << endl;
}