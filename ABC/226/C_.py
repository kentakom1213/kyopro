# 深さ優先探索

# ようやくAC
import sys
sys.setrecursionlimit(1000000)

N = int(input())
ts, seq = [0]*N, []
for i in range(N):
    t, _, *a = map(int, input().split())
    ts[i] = t
    seq.append(list(map(lambda x: x-1, a)))

Time = 0
already = [False] * N
def dfs(n):
    global Time
    if already[n]:
        return 0
    res = ts[n]
    already[n] = True
    for next in seq[n]:
        if already[next]:
            continue
        res += dfs(next)
    return res

print(dfs(N-1))

# WA
# N = int(input())
# ts, seq = [0]*N, set()
# for i in range(N):
#     t, _, *a = map(int, input().split())
#     ts[i] = t
#     seq |= set(map(lambda x: x-1, a))

# time = ts[N-1]
# for v in seq:
#     time += ts[v]

# print(time)

# TLE
# N = int(input())
# ts, seq = [0]*N, []
# for i in range(N):
#     t, _, *a = map(int, input().split())
#     ts[i] = t
#     seq.append(list(map(lambda x: x-1, a)))

# time = 0
# required = [N-1]
# already = [True] * N
# while required and any(already):
#     now = required.pop()
#     if already[now]:
#         time += ts[now]
#         already[now] = False
#     required += seq[now]  # スタックに追加

# print(time)


N = int(input())
ts, seq = [0]*N, []
for i in range(N):
    t, _, *a = map(int, input().split())
    ts[i] = t
    seq.append(list(map(lambda x: x-1, a)))

time = 0
required = [N-1]
already = [False] * N
while required and any(already):
    print(already)
    now = required.pop()
    if not already[now]:
        time += ts[now]
        already[now] = True
    required += seq[now]  # スタックに追加

print(time)