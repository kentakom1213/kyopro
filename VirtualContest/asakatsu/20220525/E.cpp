// https://atcoder.jp/contests/tenka1-2013-qualb/tasks/tenka1_2013_qualB_b

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

using pii = pair<int, int>;

int main() {
    int q, l; cin >> q >> l;
    int size = 0;
    stack<pii> lifo;

    while (q--) {
        string com; cin >> com;
        if (com == "Push") {
            int n, m; cin >> n >> m;
            lifo.push({m, n});
            size += m;
            if (size > l) {
                cout << "FULL" << endl;
                return 0;
            }
        }
        else if (com == "Pop") {
            int n; cin >> n;
            while (n > 0 && !lifo.empty()) {
                auto [x, y] = lifo.top(); lifo.pop();
                n -= y;
                size -= y;
                if (n < 0) {
                    lifo.push({x, y-n});
                    size += y-n;
                    break;
                }
            }
            if (n) {
                cout << "EMPTY" << endl;
                return 0;
            }
        }
        else if (com == "Top") {
            auto [x, y] = lifo.top();
            cout << x << endl;
        }
        else if (com == "Size") {
            cout << size << endl;
        }
    }
    cout << "SAFE" << endl;
}