//             Q1-2. お菓子 (1)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/362

// AC
// ----------------------------------------

#include <iostream>
using namespace std;

int main() {
    int N; cin >> N;
    int res = 0;

    while (N) {
        if (N % 2 == 0) N /= 2;
        else N--;
        res++;
    }
    cout << res << endl;
}