//        D - increment of coins
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc184/tasks/abc184_d

// 参考
// https://atcoder.jp/contests/abc184/editorial/152

// AC (解説)
// ----------------------------------------

// 確率DPと呼ばれるものらしい

#include <iostream>
using namespace std;
typedef long long ll;

double dp[101][101][101];

double expected_val(int a, int b, int c) {
    if (a == 100 || b == 100 || c == 100) {
        return 0;
    }
    if (dp[a][b][c]) {
        return dp[a][b][c];
    }
    else {
        double exp_v = a * (expected_val(a+1, b, c) + 1) / (a+b+c)
                     + b * (expected_val(a, b+1, c) + 1) / (a+b+c)
                     + c * (expected_val(a, b, c+1) + 1) / (a+b+c);
        return dp[a][b][c] = exp_v;
    }
}

int main() {
    ll a, b, c; cin >> a >> b >> c;
    double res = expected_val(a, b, c);
    printf("%f\n", res);
}