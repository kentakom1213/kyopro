
#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

ll N, X;
ll res = 0;
vector<vector<ll>> A;

void dfs(int i, ll prod) {
    if (i == N) {
        if (prod == X) res++;
        return;
    }

    for (ll a : A[i]) {
        if (prod*a <= X and X % a == 0) {
            dfs(i+1, a*prod);
        }
    }
}

int main() {
    cin >> N >> X;
    A.assign(N, vector<ll>());
    for (int i = 0; i < N; i++) {
        int L; cin >> L;
        for (int j = 0; j < L; j++) {
            ll a; cin >> a;
            A[i].push_back(a);
        }
    }

    dfs(0, 1);
    cout << res << endl;
}