//        D - I hate Factorization
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc166/tasks/abc166_d
// ----------------------------------------

// X > 0 であるから A > Bとして固定する
// A^5 - B^5 = (A - B)(A^4 + A^3B + A^2B^2 + A^B^3 + B^4)

// 因数分解 -> 二分探索でいけないかな


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()

int main() {
    ll X;
    cin >> X;
    
    auto A5_B5 = [](ll seg, ll b) -> ll {
        ll a = b + seg;
        return powl(a, 5) - powl(b, 5);
    };

    for (ll seg = 1; seg * seg <= X; seg++) {
        // 二分探索
        ll left = -seg, right = X;
        while (right - left > 1) {
            ll mid = (left + right) / 2;
            // printf("%lld, (%lld, %lld), res: %lld\n",seg, seg+mid, mid, A5_B5(seg, mid));
            if (A5_B5(seg, mid) <= X) left = mid;
            else right = mid;
        }

        ll res = A5_B5(seg, left);
        if (res == X) {
            // printf("%lld %lld\n", left + seg, left);
            cout << left+seg << " " << left << endl;
            break;
        }
    }
}