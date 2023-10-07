from heapq import heapify, heappop, heappush

N = int(input())
slimes = [list(map(int, input().split())) for _ in range(N)]

heapify(slimes)

cnt = 0  # マージできなくなったもの

