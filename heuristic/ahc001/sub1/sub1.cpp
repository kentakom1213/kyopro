#include <bits/stdc++.h>
using namespace std;

int main() {
    int n; cin >> n;
    vector<tuple<int, int, int>> adds(n);
    for (auto& [x, y, r] : adds) {
        cin >> x >> y >> r;
    }

    for (auto [x, y, r] : adds) {
        printf("%d %d %d %d\n", x, y, x+1, y+1);
    }
}