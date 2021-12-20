//            C - Count Order
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc150/tasks/abc150_c

// AC
// ----------------------------------------

// 8! = 40320 列挙かな

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    int N; cin >> N;
    vector<int> P(N), Q(N), perm(N);
    for (int i = 0; i < N; i++) {
        cin >> P[i];
        perm[i] = i+1;
    }
    for (int i = 0; i < N; i++) cin >> Q[i];

    int res = 0;
    bool isCount = false;
    do {
        if (isCount) res++;
        if (perm == P || perm == Q) {
            isCount = !isCount;
        }
    } while (next_permutation(perm.begin(), perm.end()));

    cout << (isCount ? 0 : res) << endl;
}