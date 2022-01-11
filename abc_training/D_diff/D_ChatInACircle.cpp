// D - Chat in a Circle
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc173/tasks/abc173_d

// 参考
// https://blog.hamayanhamayan.com/entry/2020/07/05/235409

// ほとんどうまくいっていたが、ちょっとした思い違い

// AC (解説)
// ----------------------------------------

// 貪欲法

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    vector<ll> A(N);
    for (int i = 0; i < N; i++) cin >> A[i];

    sort(A.begin(), A.end(), greater<int>());

    // 2個取る
    ll res = A[0];
    N -= 2;

    int ptr = 1, cnt = 0;
    while (N) {
        // printf("N: %d, ptr:%d, cnt:%d, res:%lld\n", N, ptr, cnt, res);
        if (cnt) {
            res += A[ptr];
            ptr++;
            cnt = 0;
        }
        else {
            res += A[ptr];
            cnt++;
        }
        N--;
    }

    cout << res << endl;
}