
# https://atcoder.jp/contests/arc148/tasks/arc148_b

N = int(input())
S = list(input())

# 区間を選び、反転して更新
ans = S[:]

for i in range(N):
    for j in range(i+1, N):
        tmp = S[:]
        tmp[i:j] = ["d" if c=="p" else "p" for c in reversed(tmp[i:j])]
        
        # 更新
        if ans > tmp:
            ans = tmp

print("".join(ans))
