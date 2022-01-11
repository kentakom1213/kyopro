from heapq import *
import heapq

N, K = map(int, input().split())
P = list(map(int, input().split()))

heap = sorted(P[:K])
print(heap[0])

for i in range(K, N):
    if P[i] > heap[0]:  # 右側に追加されるので、順位が変わる
        heappush(heap, P[i])
        heappop(heap)

    print(heap[0])

