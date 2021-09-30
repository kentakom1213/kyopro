//            2重ループの全探索 2
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/235

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

set<int> len_of_factors(int n) {
    set<int> res;
    for (int i = 1; i*i <= n; i++) {
        if (n % i == 0) {
            res.insert(i);
            res.insert(n/i);
        }
    }
    return res;
}

int main() {
    int N, K; cin >> N >> K;
    int res = 0;
    for (int i = 1; i <= N; i++) {
        if (len_of_factors(i).size() == K) res++;
    }
    cout << res << endl;
}

// for (int i = 1; i < 30; i++) {
//     cout << i << ": ";
//     for (auto f : len_of_factors(i)) cout << f << " ";
//     cout << endl;
// }