#           C - Knight Fork
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc239/tasks/abc239_c
# ----------------------------------------

def d2(a, b, c, d):
    return (a-c)**2 + (b-d)**2

def main():
    a, b, c, d = map(int, input().split())
    for x in range(a-2, a+3):
        for y in range(b-2, b+3):
            if d2(x, y, a, b) == d2(x, y, c, d) == 5:
                print("Yes")
                return
    print("No")

main()
