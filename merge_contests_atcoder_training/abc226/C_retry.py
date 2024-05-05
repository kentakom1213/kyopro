
N = int(input())
ts, seq = [0]*N, []
for i in range(N):
    t, _, *a = map(int, input().split())
    ts[i] = t
    seq.append(list(map(lambda x: x-1, a)))

time = 0
required = [N-1]
already = [False] * N
while required:
    # print(already)
    now = required.pop()
    if not already[now]:
        time += ts[now]
        already[now] = True
    required += seq[now]  # スタックに追加

print(time)