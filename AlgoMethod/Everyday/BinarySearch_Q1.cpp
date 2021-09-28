//            Q2-1. 方程式を解く
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/368

// C++って高階関数使えないの？

// AC
// ----------------------------------------

#include <iostream>
using namespace std;

bool condition(double x, int N) {
    double f = x * (x * (x + 1) + 2) + 3;
    if (f < N) return true;
    else return false;
}

double binary_search(double min, double max, int N) {
    while ((max - min) * 100000 > 1) {
        double mid = (max + min) / (double)2;
        // printf("min:%f, mid:%f, max:%f\n", min, mid, max);

        if (condition(mid, N)) min = mid;
        else max = mid;
    }
    return min;
}

int main() {
    int N; cin >> N;
    cout << binary_search(0, 100, N) << endl;
}