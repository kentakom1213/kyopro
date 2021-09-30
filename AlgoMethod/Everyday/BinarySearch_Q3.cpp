//            Q2-3. 最小の添字
// ----------------------------------------
// 問題
// https://algo-method.com/tasks/370

// 要復習かも

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, M; cin >> N >> M;
    vector<int> A(N), B(M);
    for (auto& a : A) cin >> a;
    for (auto& b : B) cin >> b;

    for (auto b : B) {
        int min = 0, max = N;
        while (max - min > 0) {
            int mid = (max + min) / 2;
            if (A[mid] >= b) max = mid;
            else min = mid + 1;  // 必ず範囲を狭める
        }
        cout << min << endl;
    }
}