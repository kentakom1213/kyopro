//           D - Water Bottle
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc144/tasks/abc144_d

// 小数を出力する問題はprintfを使おう
// そもそもPythonの方が向いている気がする

// AC
// ----------------------------------------

// 色々計算した結果
// 180 / pi * atan(a * b**2 / (2 * x))

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    double a, b, x; cin >> a >> b >> x;

    double res = 0;
    
    if (2 * x < a*a * b) {
        res = (180.0 / M_PI) * atan(a * b*b / (2 * x));
    }
    else {
        res = (180.0 / M_PI) * atan(2 * (a*a * b - x) / (a*a*a));
    }

    printf("%.10f\n", res); // 桁指定をする必要がある
}