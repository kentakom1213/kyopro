//          C - 次のアルファベット
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/code-festival-2016-quala/tasks/codefestival_2016_qualA_c

// 参考
// https://img.atcoder.jp/data/other/code-festival-2016-quala/editorial.pdf

// AC (解説)
// ----------------------------------------

// 前の文字から貪欲に

#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define ALL(A) A.begin(), A.end()
#define initArray(name, h, w, v) vector<vector<long long>> name(h, vector<long long>(w, v));
typedef long long ll;
typedef pair<long long, long long> vec2;
typedef vector<vector<int>> Array;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

int main() {
    string s; cin >> s;
    ll K; cin >> K;

    string res;

    ll s_size = s.size();
    rep(i, s_size-1) {
        int now = s[i] - 'a';
        int to_a = (26 - now) % 26;

        if (K > 0 && to_a <= K) {
            res.push_back('a');
            K -= to_a;
        } else {
            res.push_back(s[i]);
        }
    }

    // 残ったKを消費する（最後の桁で）
    if (K > 0) {
        int last = s[s_size - 1] - 'a';
        char arranged = 'a' + (last + K) % 26;
        res.push_back(arranged);
    } else {
        res.push_back(s[s_size - 1]);
    }

    cout << res << endl;
}
