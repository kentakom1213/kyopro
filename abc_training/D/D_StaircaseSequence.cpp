//        D - Staircase Sequences
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc190/tasks/abc190_d
// ----------------------------------------

// N <= 10^12 累積和ではなさそう
// sqrt(N) なら間に合う？？

// (N,) と (-N+1, -N+2, ... N) は必ず含まれる


// #include <bits/stdc++.h>
// using namespace std;
// typedef long long ll;

// int main() {
//     ll N; cin >> N;

//     // 累積和 (sqrt(N))
//     ll S[1000001];
//     for (int i = 1; i <= N; i++) {
//         S[i] = S[i-1] + i;
//         cerr << S[i] << endl;
//     }

//     ll res = 2; // (N,) と (-N+1, -N+2, ... N) は必ず含まれる

//     // 正の数のみの数列の探索
//     auto isOK = [&](int start, int end) {
//         return S[end] - S[start-1] > N;
//     };

//     for (int i = 0; i <= N; i++) {
//         // 二分探索
//         int left = -1, right = i-1;
//         while (right - left > 1) {
//             // printf("%d, %d\n", left, right);
//             int mid = (left + right) / 2;
//             if (isOK(mid, i)) left = mid;
//             else right = mid;
//         }
//         printf("r:%d, sum:%lld\n", right, S[i]-S[right-1]);
//     }
// }

// ↑見当違いも甚だしい