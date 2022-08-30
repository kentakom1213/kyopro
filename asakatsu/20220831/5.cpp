// https://atcoder.jp/contests/abc190/tasks/abc190_f

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;
constexpr ll INF = numeric_limits<long long>::max();

// BIT
struct BIT {
    int size;
    vector<int> arr;

    BIT(int n) : arr(n+1, 0) { size = n; }
    
    void add(int i, int x) {
        i++;
        while (i <= size) {
            arr[i] += x;
            i += i & -i;
        }
    }

    ll sum(int i) {
        ll res = 0;
        while (i) {
            res += arr[i];
            i -= i & -i;
        }
        return res;
    }
};

template <class T>
long long inverse_number(const vector<T>& perm) {
    int n = perm.size();
    long long sup = *max_element(perm.begin(), perm.end());
    BIT bit(sup);
    long long res = 0;
    for (int i = n-1; i >= 0; i--) {
        res += bit.sum(perm[i]);
        bit.add(perm[i], 1);
    }
    return res;
}

int main() {
    int N; cin >> N;
    vector<ll> A(N);
    rep(i, 0, N) {
        cin >> A[i];
    }
    
    ll ans = inverse_number(A);
    rep(i, 0, N) {
        cout << ans << endl;
        ans += N - 1 - A[i] * 2;
    }
}