#          D - Pair of Balls
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc216/tasks/abc216_d

# AC
# ----------------------------------------

# stackのtopをdictで管理する
# どのスタックを選ぶかをqueueで管理

from collections import deque, defaultdict

N, M = map(int, input().split())
stacks = [[] for _ in range(M)]
for i in range(M):
    input()
    stacks[i] = list(map(int, input().split()))

balls = [-1] * (N+1)
q = deque(list(range(M)))


while q:
    stack_num = q.popleft()

    top_ball = stacks[stack_num].pop()

    # ballがtopに存在しているとき
    if balls[top_ball] != -1:
        N -= 1  # ボールが消える
        # stackにまだボールがあるなら、queueに追加
        poped = balls[top_ball]
        if stacks[poped]:
            q.append(poped)
        if stacks[stack_num]:
            q.append(stack_num)
    else:
        balls[top_ball] = stack_num
    
print("No" if N else "Yes")
