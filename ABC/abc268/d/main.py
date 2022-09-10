from itertools import permutations

# solve
def separate(n, k, res=[]):
    """nのk分割を生成するジェネレータ"""
    if k == 1:
        yield res + [n]
    else:
        for i in range(1, n):
            yield from separate(n-i, k-1, res+[i])

if __name__ == "__main__":
    # input
    N, M = map(int, input().split())
    S = [input() for _ in range(N)]
    T = {input() for _ in range(M)}

    size = len("".join(S))

    for perm in permutations(S):
        for i in range(N-1, 16-size+1):

            x = "_".join(S)
            if 3 <= len(x) <= 16 and x not in T:
                print(x)
                exit()

            for sep in separate(i, N-1):
                sep.append(0)
                
                x = ""
                for i in range(N):
                    x += perm[i] + "_" * sep[i]

                if len(x) <= 16 and x not in T:
                    print(x)
                    exit()

print(-1)
