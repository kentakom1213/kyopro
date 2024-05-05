#           A - A Unique Letter           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc260/tasks/abc260_a
# ----------------------------------------

a,b,c = input()

if a==b==c:
    print(-1)
elif a==b:
    print(c)
elif a==c:
    print(b)
elif b==c:
    print(a)
else:
    print(a)