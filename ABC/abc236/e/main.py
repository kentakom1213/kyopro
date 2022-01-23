
# dp
# dp_a[i] := A[:i]からA[i]を含めて任意の数取ったときの平均値/中央値の最大値
# dp_b[i] := A[:i]からA[i]を含めず任意の数取ったときの平均値/中央値の最大値

# 更新
# dp_a[i+1] := max(
#                  dp_a
#              )
# dp_b[i+1] := 

N = int(input())
A = list(map(int, input().split()))

dp_ave = [0] * N
