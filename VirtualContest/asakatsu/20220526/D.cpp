// https://atcoder.jp/contests/agc005/tasks/agc005_a

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

int main() {
    string X; cin >> X;
    stack<char> st;
    for (char x : X) {
        if (st.size() && st.top() == 'S' && x == 'T') st.pop();
        else st.push(x);
    }

    cout << st.size() << endl;
}