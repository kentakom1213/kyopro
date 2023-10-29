//           D - String Formation          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc158/tasks/abc158_d
// ----------------------------------------

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
    string S; cin >> S;
    int Q; cin >> Q;
    
    bool head = 1;
    
    // dequeの初期値
    deque<char> ans;
    for (auto c : S) ans.push_back(c);

    while (Q--) {
        int q; cin >> q;
        if (q == 1) {
            head ^= 1;
        } else {
            int f; cin >> f;
            char c; cin >> c;

            if (head ^ (f == 1)) ans.push_back(c);
            else ans.push_front(c);
        }
    }

    if (!head) {
        reverse(ALL(ans));
    }

    for (char c : ans) cout << c;
    cout << endl;
}
