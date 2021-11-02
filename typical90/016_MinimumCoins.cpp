//         016 - Minimum Coins（★3）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_p
// ----------------------------------------

// 硬貨はだいたい貪欲法
// --> 騙された

// #include <bits/stdc++.h>
// using namespace std;
// typedef long long ll;

// int main() {
    // int N; cin >> N;
    // vector<int> coins(3);
    // for (int i = 0; i < 3; i++) cin >> coins[i];
    // sort(coins.begin(), coins.end());

//     int res = 0;
    
//     for (int i = 2; i >= 0; i--) {
//         while (N >= coins[i]) {
//             N -= coins[i];
//             res++;
//             printf("coin:%d, rest:%d\n", coins[i], N);
//         }
//     }
//     printf("rest:%d\n", N);

//     cout << res << endl;
// }


// 貪欲法 + 部分和 ?
// コインが3つしかないのには意味がある

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int n, a, b, c; cin >> n >> a >> b >> c;

    // N = Ax + By + Cz
    // z = (N - Ax - By) / C

    int P = 9999;
    int res = P + 1;
    for (int x = 0; x < P+1; x++) {
        for (int y = 0; y < P+1; y++) {
            int tmp = x * a + y * b;
            if (tmp % c != 0 || tmp > n) {
                continue;
            }

            int z = (n - tmp) / c;
            res = min(res, x+y+z);
        }
    }

    cout << res << endl;
}