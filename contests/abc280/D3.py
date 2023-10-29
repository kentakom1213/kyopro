
def factoring(n):
    result = []
    for i in range(2, int(n ** 0.5 + 1)):
        if n % i != 0: continue
        counter = 0
        while n % i == 0:
            n //= i
            counter += 1
        result.append((i, counter))
    if n != 1:
        result.append((n, 1))
    return result

def katsuage(k, n):
    l, r = -1, int(1e30)
    while (r - l) > 1:
        m = (l + r) // 2

        cnt = 0
        p = k
        while p <= m:
            cnt += m // p
            p *= k

        if cnt < n:
            l = m
        else:
            r = m
    return r


K = int(input())
fack = factoring(K)

ans = 0
for k, v in fack:
    tmp = katsuage(k, v)
    if ans < tmp:
        ans = tmp

print(ans)
