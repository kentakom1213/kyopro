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
    int N; cin >> N;
    vector<int> S(N);
    rep(i, N) cin >> S[i];
    int Q; cin >> Q;
    vector<int> queries(Q);
    rep(i, Q) cin >> queries[i];

    // ソートし、点数ごとに累積和(ランレングス圧縮)
    sort(ALL(S));
    map<int, int> border;
    int now=S[0], sum=N;
    range(i, 1, N) {
        int s = S[i];
        if (now == s) sum--;
        else {
            border[--sum] = s;
            now = s;
        }
    }
    border[N] = 0;

    print_map(border);
}
