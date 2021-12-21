#              D - Replacing
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc171/tasks/abc171_d

# AC
# ----------------------------------------

if __name__ == "__main__":
    N = int(input())
    A = list(map(int, input().split()))
    Q = int(input())

    counter = [0] * 100001

    S = sum(A)
    for a in A:
        counter[a] += 1

    for _ in range(Q):
        b, c = map(int, input().split())
        n = counter[b]
        S += (c - b) * n

        counter[b] -= n
        counter[c] += n

        print(S)