//               数字の全探索 4
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/224

// AC
// ----------------------------------------

#include <iostream>
#include <vector>
using namespace std;

int main() {
    int a, b; cin >> a >> b;
    if (a < b) swap(a, b);
    while (true) {
        a = a % b;
        if (a == 0) break;
        swap(a, b);
    }
    cout << b << endl;
}