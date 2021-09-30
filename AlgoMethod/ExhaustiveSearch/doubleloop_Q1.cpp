//            2重ループの全探索 1
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/234

// AC
// ----------------------------------------

#include <iostream>
#include <vector>
using namespace std;

bool is_prime(int n) {
    if (n == 1) return false;
    for (int i = 2; i*i <= n; i++) {
        if (n % i == 0) return false;
    }
    return true;
}

int main() {
    int N; cin >> N;
    
    int res = 0;
    for (int i = 0; i < N; i++) {
        int a; cin >> a;
        res += is_prime(a);
    }

    cout << res << endl;
}