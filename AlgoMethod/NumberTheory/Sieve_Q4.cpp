//               Q4. 双子素数
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/357

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N; cin >> N;
    vector<int> res(N+1, 1);  // 0 ~ N の配列
    res[0] = res[1] = 0;

    // 素数テーブルを作成
    for (int i = 2; i*i <= N; i++) {
        if (!res[i]) continue;
        for (int j = 2; i*j <= N; j++) {
            res[i*j] = 0;
        }
    }
    // 間隔2で走査
    int counter = 0;
    for (int i = 2; i <= N; i++) {
        if (res[i-2] && res[i]) counter++;
    }
    cout << counter << endl;
}