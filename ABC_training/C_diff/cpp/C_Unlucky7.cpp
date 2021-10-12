//             C - Unlucky 7
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc186/tasks/abc186_c

// 参考
// https://drken1215.hatenablog.com/entry/2020/12/19/224100

// 探索する範囲には気をつけよう

// AC
// ----------------------------------------

// #include <bits/stdc++.h>
// using namespace std;

// // O(logN)
// string to_oct(int n){
//   string s;
//   while(n){
//     s = to_string(n%8) + s;
//     n /= 8;
//   }
//   return s;
// }

// // O(N)
// int main() {
//     int N;
//     cin >> N;

//     int counter = 0;
//     for (int i = 1; i < N+1; i++) {
//         bool in_decimal = (to_string(i).find('7') != string::npos);
//         bool in_octal = (to_oct(i).find('7') != string::npos);
//         // printf("i:%d, 10:%d, 8:%d, %s\n", i, in_decimal, in_octal, octal_i.str().c_str());
//         if (in_decimal || in_octal) continue;
//         counter++;
//     }
//     cout << counter << endl;
// }


// けんちょんさん
#include <bits/stdc++.h>
using namespace std;

// vをb進法で表した時に7が含まれていないかどうか
bool ok(int v, int b) {
    while (v) {
        if (v % b == 7) return false;
        v /= b;
    }
    return true;
}

int main() {
    int N;
    cin >> N;

    int res = 0;
    for (int i = 1; i < N+1; i++) {
        if (ok(i, 10) && ok(i, 8)) res++;
    }

    cout << res << endl;
}