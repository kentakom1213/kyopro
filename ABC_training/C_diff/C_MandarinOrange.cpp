//          C - Mandarin Orange
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc189/tasks/abc189_c

// AC (解説)
// ----------------------------------------


// TLE
// #include <bits/stdc++.h>
// using namespace std;

// int main() {
//     int N;
//     cin >> N;
//     vector<int> A(N);
//     for (int i = 0; i < N; i++) cin >> A[i];

//     // おとなしく全探索
//     int min_num = 0;
//     int res = 0;
//     for (int i = 0; i < N; i++) {
//         for (int j = i+1; j < N+1; j++) {
//             min_num = *min_element(A.begin()+i, A.begin()+j);  // <- ここの処理がおそい？
//             res = max(res, min_num * (j - i));
//             // printf("i:%d, j:%d, min_num:%d, res:%d\n", i, j, min_num, res);
//         }
//     }
//     cout << res << endl;
// }


// AC
#include <bits/stdc++.h>
using namespace std;

int first_element_index(vector<int> &v) {
    for (int i = 0; i < v.size(); i++) {
        if (v[i]) return i;
    }
    return -1;
}


int main() {
    int N;
    cin >> N;
    vector<int> A(N);
    for (int i = 0; i < N; i++) cin >> A[i];

    // おとなしく全探索
    int min_num;
    int res = 0;
    for (int i = 0; i < N; i++) {
        int min_num = A[i];
        for (int j = i; j < N; j++) {
            min_num = min(min_num, A[j]);
            res = max(res, min_num * (j - i + 1));
        }
    }
    cout << res << endl;
}