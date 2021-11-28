#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()


int main() {
    int N, K, M;
    cin >> N >> K >> M;
    int OK = N * M;
    rep(i, N-1) {
        int a; cin >> a;
        OK -= a;
    }

    if (OK <= K) cout << max(0, OK) << endl;
    else cout << -1 << endl;
}