//         001 - Yokan Party（★4）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_a

// 参考
// https://twitter.com/e869120/status/1377027868518064129

// 初っ端からぶっ飛ばしてくるなあ

// AC
// ----------------------------------------

// 全探索すると、combi(N, K) <= combi(100000, 50000) = inf

// 解説
// K+1個の「長さがM以上のピース」に分解できるか　という問題に帰着する
// 上の命題を満たすMを二分探索で求める


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N, L, K; cin >> N >> L >> K;
    vector<int> A(N+2, 0);
    for (int i = 1; i <= N; i++) cin >> A[i];
    A[N+1] = L;

    // K+1個の「長さがM以上のピース」に分解できるか
    auto can_divide_length_M = [&](int m, int k) -> bool {
        int sep = 0;
        for (int i = 1; i < N+2; i++) {
            // printf("sep:%d, i:%d, A[i]-A[sep]:%d, K:%d\n", sep, i, A[i]-A[sep], K);
            if (A[i] - A[sep] >= m) {
                sep = i;
                k--;
            }
            if (!k) return true;           
        }
        return false;
    };
    
    // 二分探索
    int left = 0, right = L;
    while (right - left > 1) {
        int mid = (left + right) / 2;
        // printf("\nl:%d, mid:%d, r:%d\n", left, mid, right);
        if (can_divide_length_M(mid, K+1)) left = mid;
        else right = mid;
    }
    cout << left << endl;
}
