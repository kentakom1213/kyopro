//               E - Notebook              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc273/tasks/abc273_e
// ----------------------------------------

/*

## 方針
木構造を使って配列を管理する

*/

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// tree
struct Node {
    int val, par;
    vector<int> vec;

    Node (int v, int p) {
        val = v;
        par = p;
    }
};

int Q;
map<int, int> mp;
vector<Node> tree;

int main() {
    int v = 0;  // 現在の頂点
    tree.push_back(Node(-1, 0));  // 根付き木

    cin >> Q;

    string cmd;
    int x;
    while (Q--) {
        cin >> cmd;
        if (cmd == "ADD") {
            cin >> x;
            tree.push_back(Node(x, v));
            tree[v].vec.push_back((int)tree.size()-1);  // 葉を追加
            v = (int)tree.size() - 1;  // 葉に移動
        }
        else if (cmd == "DELETE") {
            v = tree[v].par;  // 親に移動
        }
        else if (cmd == "SAVE") {
            cin >> x;
            mp[x] = v;  // 現在いる頂点を保存
        }
        else if (cmd == "LOAD") {
            cin >> x;
            v = mp[x];  // 読み込み
        }
        cout << tree[v].val << " ";
    }
    cout << endl;
}
