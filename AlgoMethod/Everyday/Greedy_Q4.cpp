//             Q1-4. 荷物と箱
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/361

// AC
// ----------------------------------------

#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int main() {
    int N, M; cin >> N >> M;
    vector<int> A(N), B(M);
    for (int i = 0; i < N; i++) cin >> A[i];
    sort(A.begin(), A.end());
    for (int i = 0; i < M; i++) cin >> B[i];

    int a = 0, b = 0;
    while (a < N && b < M) {
        // 箱に入れる
        if (A[a] <= B[b]) {
            // printf("a:%d, b:%d\n", a, b);
            a++;
            b++;
        }
        // 箱をパス
        else {
            b++;
        }
    }
    cout << a << endl;
}