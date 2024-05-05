#       078 - Easy Graph Problem（★2）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_bz

# AC
# ----------------------------------------

if __name__ == "__main__":
    N, M = map(int, input().split())
    
    graph = [[] for _ in range(N)]
    for _ in range(M):
        a, b = map(int, input().split())
        a -= 1
        b -= 1
        graph[a].append(b)
        graph[b].append(a)
    
    # 探索
    cnt = 0
    for i, v in enumerate(graph):
        if len(list(filter(lambda v: v < i, v))) == 1:
            cnt += 1
    
    print(cnt)