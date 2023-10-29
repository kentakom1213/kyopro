//                A - STring               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/agc005/tasks/agc005_a
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
    string X; cin >> X;
    deque<char> st;
    for (char c : X) {
        if (st.empty() || c == 'S' || st.back() == 'T') {
            st.push_back(c);
        }
        else {
            st.pop_back();
        }
    }
    cout << st.size() << "\n";
}
