//            Q3-4. 箱の中の箱 (1)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/415

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, X; cin >> N >> X;
    vector<int> A(N-1);
    for (auto &a : A) cin >> a;

    int res = 0;
    while (X > 0) {
        res++;
        X = A[X-1];
    }
    cout << res << endl;
}