
#include <bits/stdc++.h>
using namespace std;
typedef unsigned long long ul;

vector<pair<ul, int>> factoring_pair(ul n) {
    vector<pair<ul, int>> res;
    for (ul i = 2; i*i <= n; i++) {
        int count = 0;
        while (n % i == 0) {
            count++;
            n /= i;
        }
        if (count) res.push_back({i, count});
    }
    if (n != 1) res.push_back({n, 1});
    return res;
}

int main() {
    ul n; cin >> n;

    auto res = factoring_pair(n);

    for (auto [p, n] : res) {
        printf("(%lld, %d) ", p, n);
    }
    cout << endl;
}

