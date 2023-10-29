#       C - When I hit my pocket...       
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/yahoo-procon2019-qual/tasks/yahoo_procon2019_qual_c
# ----------------------------------------

K, A, B = map(int, input().split())

if B - A <= 2 or K < A - 1:
    print(1 + K)
    exit()

# bisがAを超えるまで処理
ans = A + (B - A) * ((K - A + 1) // 2) + ((K - A + 1) % 2)
print(ans)