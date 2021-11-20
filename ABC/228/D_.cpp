
// t = 1 のクエリの計算量が O(N)であり、非効率


#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

const ll N = 1 << 20;

void insert_next(map<ll, ll>& tree, ll key, ll insert_val) {
    auto now = tree.find(key);
    ll now_v = now->second;
    now++;
    ll next_v = now->second;

    // 余裕があるとき
    if (next_v - now_v > 1) {
        tree[now_v + 1] = insert_val;
    }
    else {
        insert_next(tree, key+1, insert_val);
    }
}

int main() {
    int Q; cin >> Q;
    map<ll, ll> tree;

    rep(i, Q) {
        ll t, x; cin >> t >> x;
        ll key = x % N;
        bool is_exist = tree.find(key) != tree.end();

        if (t == 1) {
            if (is_exist) {
                insert_next(tree, key, x);
            }
            else {
                tree[key] = x;
            }
        }
        else {
            cout << (is_exist ? tree[key] : -1) << endl;
        }
    }
}