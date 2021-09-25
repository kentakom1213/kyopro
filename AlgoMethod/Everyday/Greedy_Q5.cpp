//             Q1-5. お菓子 (2)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/364

// AC
// ----------------------------------------

#include <iostream>
using namespace std;

int main() {
    int N; cin >> N;
    int res = 0;

    while (N) {
        if (N % 3 == 0) N /= 3;
        else N--;
        res++;
    }
    cout << res << endl;
}