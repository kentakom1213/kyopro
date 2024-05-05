//               E - Notebook              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc273/tasks/abc273_e
// ----------------------------------------

// 愚直な方法

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    vector<int> A = {};
    map<int, vector<int>> notebook;

    int q; cin >> q;
    while (q--) {
        string cmd; cin >> cmd;
        if (cmd == "ADD") {
            int x; cin >> x;
            A.push_back(x);
        }
        else if (cmd == "DELETE") {
            if (!A.empty()) {
                A.pop_back();
            }
        }
        else if (cmd == "SAVE") {
            int y; cin >> y;
            notebook[y] = A;
        }
        else if (cmd == "LOAD") {
            int z; cin >> z;
            A = notebook[z];
        }
        printf("%d ", (A.empty() ? -1 : A[A.size()-1]));
    }
    cout << endl;
}
