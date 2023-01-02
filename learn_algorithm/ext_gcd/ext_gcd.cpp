// 拡張ユークリッドの互除法
// https://drken1215.hatenablog.com/entry/2018/06/08/210000

#include <iostream>
using ll = long long;

// ax + by = gcd(a, b) を満たす (x, y) を求める．
int ext_gcd(ll a, ll b, ll &x, ll &y) {
    if (b == 0) {
        x = 1;
        y = 0;
        return a;
    }
    /*
    d := ext_gcd(b, a%b, t, s)
    x := t - a//b * s
    y := s
    */
    ll d = ext_gcd(b, a%b, y, x);
    y -= a / b * x;
    return d;
}

// 負数対応のmod
inline ll mod (ll a, ll m) {
    return (a % m + m) % m;
}

// 逆元計算
ll modinv(ll a, ll m) {
    ll x, y;
    ext_gcd(a, m, x, y);
    return mod(x, m);
}


int main() {
    ll a, p;
    printf("a^(-1) mod p を求めます．\n");
    printf("a: "); scanf("%lld", &a);
    printf("p: "); scanf("%lld", &p);

    ll a_inv = modinv(a, p);
    printf("aの逆元: %lld\n", a_inv);
    printf("%lld * %lld %% %lld = %lld\n",a, a_inv, p, a*a_inv%p);
}
