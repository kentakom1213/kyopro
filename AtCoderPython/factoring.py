# 参考
# https://drken1215.hatenablog.com/entry/2018/09/23/224100

def factoring(n):
    result = [n]
    while result[-1] != 0:
        f = result.pop()
        for i in range(2, int(f ** 0.5 + 1)):
            if f % i == 0:
                result += [i, f//i]
                break
        else:
            result += [f, 0]
    return result[:-1]

def fac_count(n):
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

if __name__ == "__main__":
    n = int(input())
    print(factoring(n))
    print(fac_count(n))
