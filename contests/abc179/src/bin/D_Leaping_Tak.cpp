//             D - Leaping Tak             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc179/tasks/abc179_d
// ----------------------------------------

/*
## 方針
- 累積和でdpを高速化

## 参考
- https://drken1215.hatenablog.com/entry/2020/09/20/081800
- https://atcoder.jp/contests/abc179/editorial/121
*/

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

// [modint](https://noshi91.hatenablog.com/entry/2019/03/31/174006)
#include <cstdint>
template <std::uint_fast64_t Modulus> class modint {
  using u64 = std::uint_fast64_t;

public:
  u64 a;

  constexpr modint(const u64 x = 0) noexcept : a(x % Modulus) {}
  constexpr u64 &value() noexcept { return a; }
  constexpr const u64 &value() const noexcept { return a; }
  constexpr modint operator+(const modint rhs) const noexcept {
    return modint(*this) += rhs;
  }
  constexpr modint operator-(const modint rhs) const noexcept {
    return modint(*this) -= rhs;
  }
  constexpr modint operator*(const modint rhs) const noexcept {
    return modint(*this) *= rhs;
  }
  constexpr modint operator/(const modint rhs) const noexcept {
    return modint(*this) /= rhs;
  }
  constexpr modint &operator+=(const modint rhs) noexcept {
    a += rhs.a;
    if (a >= Modulus) {
      a -= Modulus;
    }
    return *this;
  }
  constexpr modint &operator-=(const modint rhs) noexcept {
    if (a < rhs.a) {
      a += Modulus;
    }
    a -= rhs.a;
    return *this;
  }
  constexpr modint &operator*=(const modint rhs) noexcept {
    a = a * rhs.a % Modulus;
    return *this;
  }
  constexpr modint &operator/=(modint rhs) noexcept {
    u64 exp = Modulus - 2;
    while (exp) {
      if (exp % 2) {
        *this *= rhs;
      }
      rhs *= rhs;
      exp /= 2;
    }
    return *this;
  }
  friend ostream& operator<<(ostream& os, const modint& m) {
      os << m.a;
      return os;
  }
};

using mint = modint<mod>;
using pii = pair<int, int>;

int main() {
    ll N, K;
    cin >> N >> K;
    vector<pii> range(K);

    for (auto& [l, r] : range) {
        cin >> l >> r;
    }

    vector<mint> dp(N), sdp(N+1);
    dp[0] = 1;
    sdp[1] = 1;

    rep(i, 1, N) {
        for (auto p : range) {
            int left = max(0, i - p.second);
            int right = max(0, i - p.first + 1);
            dp[i] += sdp[right] - sdp[left];
        }
        sdp[i+1] = sdp[i] + dp[i];
    }
    cout << dp[N - 1] << endl;
}
