//            C - ABC Tournament
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc188/tasks/abc188_c

// AC (解説)
// ----------------------------------------

// 方針
// トーナメントの性質上、1位と2位は左半分と右半分に存在する。
// maxで1位を探し、その反対側の最大値を探せば良い


// WA  どこかで実装ミス？
/*
#include <bits/stdc++.h>
using namespace std;

int get_max_index(vector<int> &A, int start, int end) {
    int max_index = start;
    for (int i = 0; i < end; i++) {
        if (A[max_index] < A[i]) max_index = i;
    }
    return max_index;
}

int main() {
    int N;
    cin >> N;
    int pow2N = pow(2, N);
    vector<int> A(pow2N);
    for (int i = 0; i < pow2N; i++) cin >> A[i];
    
    if (get_max_index(A, 0, pow2N) < pow2N / 2) {
        cout << get_max_index(A, pow2N / 2, pow2N) + 1 << endl;
    }
    else {
        cout << get_max_index(A, 0, pow2N / 2) + 1 << endl;
    }
}
*/

// 解説を参考にして実装
// AC
#include <bits/stdc++.h>
using namespace std;

int main() {
    int N;
    cin >> N;
    vector<int> A(1 << N);
    for (int i = 0; i < (1 << N); i++) cin >> A[i];

    int half = 1 << (N - 1);
    int max = max_element(A.begin(), A.end()) - A.begin();
    
    if (max < half) {
        cout << max_element(A.begin() + half, A.end()) - A.begin() + 1 << endl;
    }
    else {
        cout << max_element(A.begin(), A.begin() + half) - A.begin() + 1 << endl;
    }
}