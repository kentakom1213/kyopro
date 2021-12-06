//            D - Triangles
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc143/tasks/abc143_d

// AC
// ----------------------------------------

// a < b + c
// b < c + a
// c < a + b

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> point;
typedef vector<vector<int>> Array;

int main() {
    int N; cin >> N;
    vector<int> bars(N);
    rep(i, N) cin >> bars[i];
    sort(ALL(bars));

    int res = 0;
    for (int i = 0; i < N; i++) {
        for (int j = i+1; j < N; j++) {
            // 2辺の和が1辺の長さを超えてはならない
            int max_len = bars[i] + bars[j];
            // printf("a:%d, b:%d, max:%d\n",bars[i], bars[j], max_len);

            // 条件を満たすkを2分探索で求める
            int l = j, r = N;
            while ((r - l) > 1) {
                int mid = (l + r) / 2;
                if (bars[mid] < max_len) l = mid;
                else r = mid;
            }
            res += (l - j);
        }
    }

    cout << res << endl;
}