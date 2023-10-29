N, T = input().split()
N = int(N)

def isOK(S):
    """等しい可能性があるか判定する"""
    is_equal = True
    cnt_insert = 0
    cnt_delete = 0
    cnt_change = 0

    s, t, t1, t2 = 0, 0, 0, 0
    while (s < len(S) and t < len(T) and t1 < len(T)):
        if S[s] != T[t]:
            is_equal = False
            cnt_change += 1

        if S[s] != T[t1]:
            # 進める
            t1 += 1
            cnt_insert += 1
        
        if S[s] != T[t2]:
            # 戻す
            t2 -= 1
            cnt_delete += 1
        
        s = min(s + 1, len(S))
        t = min(t + 1, len(T))
        t1 = min(t1 + 1, len(T))
        t2 = min(t2 + 1, len(T))

    print(S, is_equal, cnt_insert, cnt_delete, cnt_change)

    return (is_equal
            or cnt_insert <= 1
            or cnt_delete <= 1
            or cnt_change <= 1)

ans = []

for i in range(N):
    s = input()
    if isOK(s):
        ans.append(i + 1)

# 答え
print(len(ans))
print(*ans)
