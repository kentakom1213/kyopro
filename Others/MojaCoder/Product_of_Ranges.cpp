// Product of Ranges
// https://mojacoder.app/users/powell/problems/product_of_ranges

#include <iostream>
#include <vector>

#include <vector>
using namespace std;

using ll = long long;
constexpr ll MOD = 998244353;

template<typename T>
T powmod(T x, T n) {
    T ret = 1;
    while (n > 0) {
        if (n & 1) ret = ret * x % MOD;  // n の最下位bitが 1 ならば x^(2^i) をかける
        x = x * x % MOD;
        n >>= 1;  // n を1bit 左にずらす
    }
    return ret;
}

ll inv(ll x) {
    return powmod(x, MOD-2);
}

// BIT (積version)
struct BIT {
    int size;
    vector<ll> arr;

    BIT(int n) : arr(n+1, 1) { size = n; }
    
    void mul(int i, ll x) {
        while (i <= size) {
            arr[i] = (arr[i] * x) % MOD;
            i += i & -i;
        }
    }

    int prod(int i) {
        ll res = 1;
        while (i) {
            res = (res * arr[i]) % MOD;
            i -= i & -i;
        }
        return res;
    }
};

int main() {
    int N, Q; cin >> N >> Q;
    BIT bit(N);

    // 初期化
    for (int i = 1; i <= N; i++) {
        int a; cin >> a;
        bit.mul(i, a);
    }

    // クエリ処理
    while (Q--) {
        int l, r; cin >> l >> r;
        cout << bit.prod(r) * inv(bit.prod(l-1)) % MOD << endl;
    }

    return 0;
}
