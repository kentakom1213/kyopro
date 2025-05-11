N, M = map(int, input().split())
A = list(map(int, input().split()))

correct = set(range(1, M + 1))

ans = 0

for _ in range(N):
    if not (set(A) >= correct):
        print(ans)
        exit()

    # 末尾を削除
    A.pop()

    ans += 1

if not (set(A) >= correct):
    print(ans)
