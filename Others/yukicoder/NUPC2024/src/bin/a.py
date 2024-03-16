K = int(input())

# 解答
def decode(n):
    T = []
    for i, c in enumerate("NUPC"):
        T.append(c)
        if (n >> i) & 1:
            T *= 2
    return "".join(T)

SS = sorted(map(decode, range(16)))

print(SS[K - 1])
