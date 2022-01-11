//             D - Alter Alter
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc174/tasks/abc174_d
// ----------------------------------------

// 例)
// before: WRWWWRRW
// after : RRRWWWWW
// before -> afterにするための最小回数を求める

// #include <bits/stdc++.h>
// using namespace std;
// typedef long long ll;

// int main() {
//     int N; cin >> N;
//     string S; cin >> S;

//     int cnt = 0;
//     int pw = 0, pr = N - 1;
//     while (pw < pr) {
//         if (S[pw] == 'W' && S[pr] == 'R') {
//             pw++, pr--;
//             cnt++;
//         }
//         else if (S[pw] == 'W') pr--;
//         else if (S[pr] == 'R') pw++;
//     }

//     cout << cnt << endl;
// }

// TLE


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    string S; cin >> S;

    int temp, cnt = 0;
    int pw = 0, pr = N - 1;
    while (pw <= pr) {
        temp = 0;
        if (S[pw] == 'W') {
            pr--;
            temp++;
        }
        if (S[pr] == 'R') {
            pw++;
            temp++;
        }
        printf("pw:%d, pr:%d, temp:%d, cnt:%d\n", pw, pr, temp, cnt);  // debug
        cnt += temp / 2;
    }

    cout << cnt << endl;
}
