from collections import deque

N = int(input())
A = list(map(int, input().split()))

# ソート
dq = deque(sorted(A))

now = 0
stock = 0  # 売れる漫画の数
while dq:
    top = dq[0]
    if top == now:
        dq.popleft()
        stock += 1
    elif top == now + 1:
        dq.popleft()
        now += 1
    else:
        if stock >= 2:
            now += 1
            stock -= 2
        else:
            dq.pop()
            stock += 1
    
print(now + stock // 2)
