//      D - I Hate Non-integer Number      
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc262/tasks/abc262_d

// 要復習
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;
#define rep(i, a, b) for (int i = (int)(a); i < (int)(b); i++)
#define ALL(A) A.begin(), A.end()
using ll = long long;
template <typename T> using Array = vector<vector<T>>;
template <typename T> inline bool chmax(T &a, const T b) { if (a < b) { a = b; return true; } return false; }
template <typename T> inline bool chmin(T &a, const T b) { if (a > b) { a = b; return true; } return false; }
using pii = pair<int, int>;
using pll = pair<ll, ll>;
constexpr int MOD = 1000000007;
constexpr int mod = 998244353;

// [modint](https://noshi91.hatenablog.com/entry/2019/03/31/174006)
#include <cstdint>
template <std::uint_fast64_t Modulus>
class modint {
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

int main() {
    int N; cin >> N;
    vector<int> A(N);
    rep(i, 0, N) cin >> A[i];

    // 更新
    mint ans = 0;

    rep(i, 1, N+1) {
        vector dp(101, vector(101, vector<mint>(101, 0)));
        dp[0][0][0] = 1;
        rep(j, 0, N) {
            rep(k, 0, i+1) {
                rep(l, 0, i) {
                    dp[j+1][k][l] += dp[j][k][l];
                    if (k!=i) dp[j+1][k+1][(l+A[j])%i] += dp[j][k][l];
                }
            }
        }
        ans += dp[N][i][0];
    }
    cout << ans << endl;
}