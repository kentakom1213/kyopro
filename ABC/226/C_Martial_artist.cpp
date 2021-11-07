#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    vector<int> ts(N);
    vector<vector<int>> seq(N);
    for (int i = 0; i < N; i++) {
        int t; cin >> t;
        ts[i] = t;
        int k; cin >> k;
        for (int j = 0; j < k; j++) {
            int a; cin >> a;
            seq[i].push_back(a-1);
        }
    }

    ll time = 0;
    vector<bool> already(N, false);
    stack<int> required;
    required.push(N-1);
    while (!required.empty()) {
        int now = required.top();
        required.pop();
        for (int v : seq[now]) required.push(v);
        if (!already[now]) {
            time += ts[now];
            already[now] = true;
        }
    }

    cout << time << endl;
}