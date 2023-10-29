
#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, Q; cin >> N >> Q;
    vector<long long> A(N);
    for (auto &a : A) cin >> a;

    // 累積和
    vector<long long> S(N+1, 0);
    for (int i=0; i < N; i++) {
         S[i+1] = S[i] + A[i];
    }

    // クエリ処理
    while (Q--) {
        int l, r; cin >> l >> r;
        l--;

        cout << S[r] - S[l] << endl;
    }
}
