
// 解説

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

constexpr int N = 1 << 20;
constexpr int MASK = N - 1;  // 11111111111111111111

int main() {
    int Q; cin >> Q;
    map<ll, ll> interval, tree;
    interval[N] = 0;

    while (Q--) {
        ll t, x; cin >> t >> x;
        if (t == 1) {
            int i = x & MASK;
            auto itr = interval.upper_bound(i);  // iより大きい最初のイテレータ
            if (itr == interval.end()) {
                i = 0;
                itr = interval.begin();
            }
            if (itr->second <= i) {
                tree[i] = x;
                int l1 = itr->second, r1 = i, l2 = i+1, r2 = itr->first;
                interval.erase(itr);
                if (l1 != r1) {
                    interval[r1] = l1;
                }
                if (l2 != r2) {
                    interval[r2] = l2;
                }
            }
            else {
                tree[itr->second] = x;
                itr->second += 1;
                if (itr->first == itr->second) {
                    interval.erase(itr);
                }
            }
        }
        else {
            auto itr = tree.find(x & MASK);
            cout << (itr == tree.end() ? -1 : itr->second) << endl;
        }
    }
}