def separate(n, k, res=[]):
    """nのk分割を生成"""
    if k == 1:
        yield res + [n]
    else:
        for i in range(1, n):
            yield from separate(n-i, k-1, res+[i])


if __name__ == "__main__":
    n, k = map(int, input().split())
    
    for x in separate(n, k):
        print(x)
