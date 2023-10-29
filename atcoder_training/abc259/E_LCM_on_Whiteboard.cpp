//          E - LCM on Whiteboard          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc259/tasks/abc259_e
// ----------------------------------------

/*
## 参考
- https://atcoder.jp/contests/abc259/editorial/4271
*/

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template<typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
template <typename A, size_t N, typename T> void FILL(A (&array)[N], const T &val) { fill( (T*)array, (T*)(array+N), val); }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

using Num = map<int, int>;  // {素因数:次数}

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
    vector<Num> nums(N);
    
    for (auto &num : nums) {
        int m; cin >> m;
        rep(i, 0, m) {
            int e, p; cin >> e >> p;
            num[e] = p;
        }
    }

    // 全ての数の最小公倍数を求める
    Num LCM_ALL;
    for (auto num : nums) {
        for (auto [e, m] : num) {
            if (LCM_ALL.find(e) == LCM_ALL.end()) {
                LCM_ALL[e] = m;
            }
            else {
                chmax(LCM_ALL[e], m);
            }
        }
    }

    // 順に消していき，最小公倍数としてあり得る数を求める
    set<Num> lcms;
    for (auto num : nums) {
        Num lcm_i = LCM_ALL;  // C++の代入は値渡し？
        for (auto [e, m] : num) {
            lcm_i[e] -= m;
            if (lcm_i[e] == 0) {
                lcm_i.erase(lcm_i.find(e));
            }
        }
        print_map(lcm_i);
        lcms.insert(lcm_i);
    }

    cout << lcms.size() << endl;
}
