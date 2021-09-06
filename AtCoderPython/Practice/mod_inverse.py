# 参考
# https://drken1215.hatenablog.com/entry/2018/06/08/210000
# https://qiita.com/drken/items/b97ff231e43bce50199a
# https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a

#     a ^ (-1) mod p
# <=> ax + py = 1
# を満たす (x. y) を求める。


### 再帰関数による実装 ###
# ac + by = gcd(a, b) = d
# a = qb + r ...(i)
#
#     (qb + r)x + by = d
# <=> b(qx + y) + rx = d ...(ii)
# <=> bs + rt = d
#   (s, t) が求まったとき
#   (x, y) = (t, s - qt)
#
#   終了条件
# <=> dx + 0y = d <=> (x, y) = (1, 0) ...(iii)
#######################

def extGCD(a, b):
    if b == 0:  # (iii)
        return (1, 0)  # -> (x, y)

    q, r = a//b, a % b  # (i)
    s, t = extGCD(b, r)
    return (t, s - q * t)  # -> (x, y)


def modInv(a, p):
    if a > p: return modInv(a % p, p)
    if a == 1: return 1  # mod(ax, p) = 1 => x = 1
    return p - modInv(p % a, p) * (p // a) % p


# 高速 mod_inverse
# ax + py = 1
MOD = int(1e9 + 7)

def init_inv(max, mod):
    inv = [0] * (max % mod)
    inv[1] = 1
    for i in range(2, max % mod):
        inv[i] = mod - inv[mod % i] * (mod // i) % mod
        print(inv[i])
    return inv


if __name__ == "__main__":
    inv = init_inv(210000, MOD)
    print(inv[:100])
