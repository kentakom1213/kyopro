// https://atcoder.jp/contests/abc194/tasks/abc194_e

#include <bits/stdc++.h>
using namespace std;
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

template<class T>
void print_set(set<T> S) {
    cout << "{ ";
    for (auto val : S) {
        cout << val << ", "[val == *(--S.end())];
    }
    cout << "}" << endl;
}

int main() {
    int N, M; cin >> N >> M;
    vector<int> A(N);
    for (int &a: A) cin >> a;

    // 区間を保存
    set<int> range;
    for (int i=0; i<M; i++) {
        range.insert(A[i]);
    }

    int mex_min = range.

}
