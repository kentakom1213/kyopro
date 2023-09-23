
#include <bits/stdc++.h>
using namespace std;

int MOD = 998244353;

long long powmod(long long a, long long n) {
    long long res = 1;
    while (n > 0) {
        if (n & 1) res = (res * a) % MOD;
        a = (a * a) % MOD;
        n >>= 1;
    }
    return res;
}

long long inv(long long n) {
    return powmod(n, MOD-2);
}

int main() {
    int N, Q; cin >> N >> Q;
    vector<long long> A(N);
    for (auto &a : A) cin >> a;

    // 累積積
    vector<long long> S(N+1, 1);
    for (int i=0; i < N; i++) {
         S[i+1] = S[i] * A[i] % MOD;
    }

    // クエリ処理
    while (Q--) {
        int l, r; cin >> l >> r;
        l--;

        cout << S[r] * inv(S[l]) % MOD << endl;
    }
}
