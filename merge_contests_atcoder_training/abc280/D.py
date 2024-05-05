from collections import defaultdict

def factoring(n):
    result = defaultdict(int)
    for i in range(2, int(n ** 0.5 + 1)):
        if n % i != 0: continue
        while n % i == 0:
            n //= i
            result[i] += 1
    if n != 1:
        result[n] += 1
    return result

def is_subseteq(a, b):
    """ `a <= b` """
    is_ok = True
    for k in a.keys():
        is_ok &= a[k] <= b[k]
    return is_ok

def main():
    K = int(input())

    fac_K = factoring(K)
    
    factorial = defaultdict(int)
    for i in range(1, int(1e20)):
        # factorial に追加
        for k, v in factoring(i).items():
            factorial[k] += v
        
        if is_subseteq(fac_K, factorial):
            print(i)
            return

main()
