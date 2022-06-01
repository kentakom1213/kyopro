// https://atcoder.jp/contests/code-formula-2014-quala/tasks/code_formula_2014_qualA_b

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

void print_pins(vector<char> p) {
    printf("%c %c %c %c\n %c %c %c\n  %c %c\n   %c\n", p[7], p[8], p[9], p[0], p[4], p[5], p[6], p[2], p[3], p[1]);
}

int main() {
    int a, b; cin >> a >> b;
    vector<char> pins(10, 'x');
    rep(i, 0, a) {
        int p; cin >> p;
        pins[p] = '.';
    }
    rep(i, 0, b) {
        int p; cin >> p;
        pins[p] = 'o';
    }

    print_pins(pins);
}
