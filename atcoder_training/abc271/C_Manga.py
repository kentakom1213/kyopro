#                C - Manga                
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc271/tasks/abc271_c
# ----------------------------------------

# 解の存在で二分探索

N = int(input())
A = set(map(int, input().split()))

ok, ng = 0, N+1
while ng-ok > 1:
    mid = (ok + ng) // 2
    C = len(set(range(1, mid+1)) & A)  # 共通部分の判定 O(N)
    isOK = C + (N-C) // 2 >= mid
    if isOK:
        ok = mid
    else:
        ng = mid

print(ok)
