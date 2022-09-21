//                A - A ↔ BB               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc136/tasks/arc136_a
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
    int N; cin >> N;
    string S; cin >> S;

    string ans;
    for (char c : S) {
        if (ans.size() > 0) {
            if (ans.back() == 'B' && c == 'A') {
                ans.pop_back();
                ans.push_back('A');
                ans.push_back('B');
            } else if (ans.back() == 'B' && c == 'B') {
                ans.pop_back();
                ans.push_back('A');
            } else {
                ans.push_back(c);
            }
        } else {
            ans.push_back(c);
        }
    }

    cout << ans << endl;
}
