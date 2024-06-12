#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

int main() {
    int T; cin >> T;
    rep(t, T) {
        ll a, s; cin >> a >> s;
        ll rest = s - a*2;
        
        // そもそも成立しない
        if (rest < 0) {
            cout << "No" << endl;
            continue;
        }

        bool isOK = true;
        for (int i=0; i <= 60; i++) {
            if (((rest >> i) & 1) && ((a >> i) & 1)) {
                isOK = false;
            }
        }
        cout << (isOK ? "Yes" : "No") << endl;
    }
}