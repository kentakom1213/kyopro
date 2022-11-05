# https://atcoder.jp/contests/arc118/tasks/arc118_b

N, K, M = map(int, input().split())
A = list(map(int, input().split()))
sumA = sum(A)

F = [(a*M/sumA) for a in A]
diff = sorted((f, i) for f, i in zip(F, range(N)))

ans = [0] * N
rem = M
while diff:
    (f, i) = diff.pop()
    if len(diff):
        # 四捨五入
        f45 = int(f) + (1 if f-int(f) >= 0.5 else 0)
        rem -= f45
        ans[i] = f45
    else:
        ans[i] = rem

print(*ans)
