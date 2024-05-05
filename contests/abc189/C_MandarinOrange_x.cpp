//          C - Mandarin Orange
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc189/tasks/abc189_c
// ----------------------------------------

// 方針
// N <= 10^4 であるから、全ての(l, r) -> O(n^2) は無理
// 連続した区間であるから尺取り法で探索

#include <bits/stdc++.h>
using namespace std;

int first_element_index(vector<int> &v) {
    for (int i = 0; i < v.size(); i++) {
        if (v[i]) return i;
    }
    return 0;
}

int last_element_index(vector<int> &v) {
    int index = 0;
    for (int i = 0; i < v.size(); i++) {
        if (v[i]) index = i;
    }
    return index;
}


int inchworm(vector<int> &A) {
    vector<int> nums(100000);
    int res = 0;
    int r = 0;
    int max_num = 0;

    for (int l = 0; l < A.size(); l++) {
        // max_num = last_element_index(nums);
        while (r < A.size() && first_element_index(nums) <= A[r]) {
            nums[ A[r] ]++;
            r++;
        }

        res = max(res, first_element_index(nums) * (r - l));

        // printf("l:%d, r:%d, min_num:%d, max_num:%d ", l, r, first_element_index(nums), last_element_index(nums));
        // print_vector(nums);

        if (l == r) r++;
        nums[ A[l] ]--;
    }

    return res;
}


int main() {
    int N;
    cin >> N;
    vector<int> A(N);
    for (int i = 0; i < N; i++) cin >> A[i];

    cout << inchworm(A) << endl;
}