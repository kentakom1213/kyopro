//               E - Max Min               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc247/tasks/abc247_e

// 自力実装は無理な気がする

// AC
// ----------------------------------------

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

ll N, X, Y;
ll A[202020];

template <typename T>
void print_vector(vector<T>& vec) {
  cerr << "[ ";
  for (int i = 0; i < vec.size(); i++) {
    if (i < vec.size() - 1) cerr << vec.at(i) << " ";
    else cerr << vec.at(i);
  }
  cerr << " ] ";
}

// 区間Bの連続部分列の中で、X, Yを含む部分列の個数をカウント
ll cnt_range(vector<ll> B) {
    ll Bs=B.size(), l=0, r=0, cntX=0, cntY=0, res=0;
    while (l < Bs) {
        while (r < Bs && (cntX==0 || cntY==0)) {
            if (B[r]==X) cntX++;
            if (B[r]==Y) cntY++;
            r++;
        }
        if (cntX && cntY) {
            print_vector(B);
            printf("%lld:%lld\n", l, r);
            res += Bs - r + 1;
        }
        if (B[l]==X) cntX--;
        if (B[l]==Y) cntY--;
        l++;
    }
    return res;
}

int main() {
    cin >> N >> Y >> X;
    rep(i, 0, N) cin >> A[i];

    ll ans = 0;

    // B = {b | X <= b <= Y} を求める : 尺取法
    vector<ll> B;
    for (auto b : A) {
        if (X <= b && b <= Y) {
            B.push_back(b);
        }
        else if (B.size()) {
            ans += cnt_range(B);
            B.clear();
        }
    }
    if (B.size()) {
        ans += cnt_range(B);
    }

    cout << ans << endl;
}
