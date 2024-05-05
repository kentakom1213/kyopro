from collections import defaultdict

def factoring(n):
    N = n
    result = defaultdict(int)
    for i in range(2, N):
        if i*i > N: break
        if n % i != 0: continue
        while n % i == 0:
            n //= i
            result[i] += 1
    if n != 1:
        result[n] += 1
    return result

def katsuage(k, v):
    """ 「因数`k`が`v`個含まれるのは最小で`N`の階乗である」 """
    l, r = 0, 1e100
    while (r - l) > 1:
        m = (l + r) // 2
        if v <= m * (m+1) // 2:
            r = m
        else:
            l = m
    return int(r * k)

def main():
    K = int(input())

    facK = factoring(K)

    ans = 0
    for k, v in facK.items():
        tmp = katsuage(k, v)
        ans = max(ans, tmp)
    
    print(ans)

main()
