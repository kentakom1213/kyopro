//              C - 説明会
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/indeednow-quala/tasks/indeednow_2015_quala_3
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define range(i, begin, end) for (int i = (int)(begin); i < (int)(end); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<int>> name(h, vector<int>(w, v));
typedef long long ll;
typedef pair<int, int> vec2;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

template<class T, class U>
void print_map(map<T, U> dict) {
    cerr << "{\n";
    for (auto &[a, b] : dict) {
        cerr << "   {" << a << ", " << b << "},\n";
    }
    cerr << "}" << endl;
}


int main() {
    int n, N; cin >> n;
    vector<int> S;
    rep(i, n) {
        int s; cin >> s;
        if (s) {
            S.push_back(s);
            N++;
        }
    };

    // ソートし、点数ごとに累積和(ランレングス圧縮)
    sort(ALL(S), greater<int>());
    map<int, int> border;
    int now=S[0], sum=0;
    range(i, 1, N) {
        int s = S[i];
        if (now == s) sum++;
        else {
            border[++sum] = now;
            now = s;
        }
    }
    border[N] = S[N-1];

    print_map(border);

    int Q; cin >> Q;
    while(Q--) {
        int q; cin >> q;
        auto ans = border.lower_bound(q);
        cout << ans->first << " " << ans->second << endl;
    }
}
