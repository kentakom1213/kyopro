
#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, Q; cin >> N >> Q;
    vector<long long> A(N);
    for (auto &a : A) cin >> a;

    // クエリ処理
    while (Q--) {
        int l, r; cin >> l >> r;
        l--;

        // 和をとる
        long ans = 0;
        for (int j = l; j < r; j++) {
            ans += A[j];
        }

        cout << ans << endl;
    }
}
