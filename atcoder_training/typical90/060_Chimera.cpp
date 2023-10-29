//            060 - Chimera（★5）            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bh
// ----------------------------------------

/*
## 方針
- 両側からLISを取る。
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
typedef long long ll;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

const int INF = 1 << 30;
const int MAX_SIZE = 303030;

template <typename T>
void print_vector(const vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ]" << endl;
}

vector<int> cnt_LIS(vector<int>& arr) {
    vector<int> lis(arr.size(), INF), res(arr.size(), INF);
    int len_of_lis = 0;
    for (int i = 0; i < arr.size(); i++) {
        auto ptr = lower_bound(ALL(lis), arr[i]);
        if (*ptr == INF) len_of_lis++;
        res[i] = len_of_lis;
        chmin(*ptr, arr[i]);
    }
    return res;
}

int main() {
    int N; cin >> N;
    vector<int> A(N), revA(N);
    rep(i, 0, N) {
        cin >> A[i];
        revA[N-i-1] = A[i];
    }

    // 左右でLISをとる
    vector<int> lis_left = cnt_LIS(A);
    vector<int> lis_right = cnt_LIS(revA);
    reverse(ALL(lis_right));

    int ans = 0;
    for (int i = 0; i < N; i++) {
        chmax(ans, lis_left[i] + lis_right[i] - 1);
    }
    cout << ans << endl;
}
