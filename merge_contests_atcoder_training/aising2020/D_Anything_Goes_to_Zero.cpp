//        D - Anything Goes to Zero        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/aising2020/tasks/aising2020_d
// ----------------------------------------

#include<bits/stdc++.h>
using namespace std;

#define rep(i, a, b) for (int i = a; i < b; i++)
#define all(vec) vec.begin(), vec.end()

int N;
string X;
int cnt[201010];

int main() {
    cin >> N >> X;

    rep(i, 1, 201010) {
        int pp = __builtin_popcount(i);
        cnt[i] = cnt[i % pp] + 1;
    }

    reverse(all(X));

    int cn = 0;
    rep(i, 0, N) if (X[i] == '1') cn++;

    int tot1 = 0, p1 = 1;
    int tot2 = 0, p2 = 1;
    rep(i, 0, N) {
        if (X[i] == '1') {
            tot1 = (tot1 + p1) % (cn + 1);
            if (2 <= cn) tot2 = (tot2 + p2) % (cn - 1);
        }

        p1 = (p1 * 2) % (cn + 1);
        if (2 <= cn) p2 = (p2 * 2) % (cn - 1);
    }

    vector<int> ans;
    p1 = 1;
    p2 = 1;
    rep(i, 0, N) {
        if (X[i] == '1') {
            if (cn == 1) {
                ans.push_back(0);
            }
            else {
                int x = (tot2 - p2 + cn - 1) % (cn - 1);
                ans.push_back(cnt[x] + 1);
            }
        }
        else {
            int x = (tot1 + p1 + cn + 1) % (cn + 1);
            ans.push_back(cnt[x] + 1);
        }

        p1 = (p1 * 2) % (cn + 1);
        if (2 <= cn) p2 = (p2 * 2) % (cn - 1);
    }

    reverse(all(ans));
    rep(i, 0, N) printf("%d\n", ans[i]);
}
