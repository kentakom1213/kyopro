# https://atcoder.jp/contests/abc177/tasks/abc177_b

# 全探索
S, T = input(), input()
ls, lt = len(S), len(T)

ans = 1e20
for i in range(ls-lt+1):
    tmp = 0
    for j in range(lt):
        tmp += S[i+j] != T[j]
    if ans > tmp:
        ans = tmp

print(ans)