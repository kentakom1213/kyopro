//         C - Factors of Factorial
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc067/tasks/arc067_a

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
const ll MOD = 1000000007;

typedef long long ll;
vector<pair<int, int>> factoring_pair(int n) {
    vector<pair<int, int>> res;
    for (int i = 2; i*i <= n; i++) {
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
    int N; cin >> N;
    map<int, int> factors;
    for (int i = 2; i <= N; i++) {
        vector<pair<int, int>> fs = factoring_pair(i);
        for (auto& [p, n] : fs) {
            factors[p] += n;
        }
    }

    ll res = 1;
    for (auto itr = factors.begin(); itr != factors.end(); itr++) {
        // cerr << itr -> first << " : " << itr -> second << endl;
        res = (res * (itr -> second + 1)) % MOD;
    }

    cout << res << endl;
}