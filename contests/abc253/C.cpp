
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
    int Q; cin >> Q;
    multiset<int> s;

    while (Q--) {
        int q; cin >> q;
        if (q == 1) {
            int x; cin >> x;
            s.insert(x);
        }
        else if (q == 2) {
            int x, c; cin >> x >> c;
            rep(i, 0, c) {
                auto itr = s.find(x);
                if (itr != s.end()) {
                    s.erase(itr);
                } else {
                    break;
                }
            }
        }
        else {
            int minV=*s.begin(), maxV=*s.rbegin();
            cout << maxV - minV << endl;
        }
    }
}
