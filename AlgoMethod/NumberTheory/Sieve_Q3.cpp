//          Q3. N 以下の素数の個数
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/330

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; cin >> N;
    vector<int> res(N+1, 1);  // 0 ~ N の配列
    res[0] = 0; res[1] = 0;

    for (int i = 2; i*i <= N; i++) {
        if (!res[i]) continue;
        for (int j = 2; i*j <= N; j++) {
            res[i*j] = 0;
        }
    }
    cout << accumulate(res.begin(), res.end(), 0) << endl;
}