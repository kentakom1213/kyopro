#         A - Reverse and Minimize        
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc142/tasks/arc142_a
# ----------------------------------------

N, K = input().split()
N = int(N)

if K > K[::-1]:
    print(0)
    exit()

st = set()
for i in range(12):
    x = int(K + "0" * i)
    y = int(K[::-1] + "0" * i)
    if x <= N:
        st.add(x)
    if y <= N:
        st.add(y)

print(len(st))
