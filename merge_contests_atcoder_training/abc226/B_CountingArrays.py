
# AC

if __name__ == "__main__":
    N = int(input())
    seq = set()
    for i in range(N):
        _, *a = tuple(map(int, input().split()))
        seq.add(tuple(a))

    print(len(seq))