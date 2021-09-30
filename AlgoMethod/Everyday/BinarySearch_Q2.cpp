//              Q2-2. 貯金 (1)
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/367

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M; cin >> N >> M;

    auto isOK = [&](double x) {
        double deposit = x * (x * (x * (x * (x * (N + 1) + 1) + 1) + 1) + 1) + 1;
        return deposit < M;
    };

    double min = 0, max = 10000;
    while ((max - min) * 1000 > 1) {
        double mid = (max + min) / (double)2;
        // printf("%f, %f, %f\n", min, mid, max);
        if (isOK(mid)) min = mid;
        else max = mid;
    }
    cout << min << endl;
}