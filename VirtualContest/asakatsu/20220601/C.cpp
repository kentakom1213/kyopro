// https://atcoder.jp/contests/abc017/tasks/abc017_2

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
    string S; cin >> S;
    bool isOK = true;
    for (int i=0; i<S.size(); i++) {
        char c = S[i];
        if (c=='o' || c=='k' || c=='u') {
            continue;
        } else if (c=='c') {
            isOK &= i<S.size() && S[i+1] == 'h';
        } else if (c=='h') {
            isOK &= i>=1 && S[i-1] == 'c';
        } else {
            isOK = false;
        }
    }
    cout << (isOK ? "YES" : "NO") << endl;
}
