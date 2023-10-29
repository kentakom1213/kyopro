#         D - Happy Birthday! 2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc200/tasks/abc200_d

# AC
# ----------------------------------------

# 部分和問題と考えるとDPが使える
# dp復元を使って、配列を復元する

# こうして世紀のゴミコードが生まれた...

import sys
def err(*args, **kwargs): print(*args, **kwargs, file=sys.stderr)
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
A = [int(x)%200 for x in input().split()]

### 非dpで解ける部分
## 0 or 200があるとき
if 0 in A:
    zero = A.index(0)
    not_zero = (zero + 1) % N
    print("Yes")
    print(1, not_zero+1)
    if zero < not_zero:
        print(2, zero+1, not_zero+1)
    else:
        print(2, not_zero+1, zero+1)
    exit()

## 重複を調べる
A_sorted = sorted(A)
dup = None
pre = None
for n in A_sorted:
    if pre == n:
        dup = n
        break
    pre = n

if dup != None:
    err("DUP")
    print("Yes")
    print(1, (fst:=A.index(dup))+1)
    A[fst] = -1
    print(1, A.index(dup)+1)
    exit()

### dpによる解法
dp = init_array(N+1, 201)
dp[0][0] = 1

for i in range(N):
    for j in range(200):
        if dp[i][j] == 0: continue

        dp[i+1][j] += dp[i][j]
        dp[i+1][(j+A[i])%200] += dp[i][j]

err(*dp, sep="\n")

dup = []
for i, n in enumerate(dp[-1]):
    if i and n >= 2:
        dup.append(i)

# dp復元
def dp_recover(last):
    B, C = [last], [last]
    for i in range(N-1, -1, -1):

        b = B[-1]
        if dp[i][(b-A[i])%200]:
            B.append((b-A[i])%200)
        else:
            B.append(b)
        
        c = C[-1]
        if dp[i][c]:  # ルートを変える
            C.append(c)
        else:
            C.append((c-A[i])%200)
    
    B.reverse()
    C.reverse()

    # 差分をとる
    B_diff = [(B[i+1]-B[i])%200 for i in range(N)]
    C_diff = [(C[i+1]-C[i])%200 for i in range(N)]

    return B_diff, C_diff

for last in dup:
    B_diff, C_diff = dp_recover(last)
    if B_diff != C_diff:
        break
else:
    print("No")
    exit()

### 出力
print("Yes")

print(sum(map(bool, B_diff)), end=" ")
for i in range(N):
    if B_diff[i]:
        print(i+1, end=" ")
print()

print(sum(map(bool, C_diff)), end=" ")
for i in range(N):
    if C_diff[i]:
        print(i+1, end=" ")
print()
