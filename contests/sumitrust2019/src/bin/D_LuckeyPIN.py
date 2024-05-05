#            D - Luckey PIN
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d
# ----------------------------------------

# 愚直に調べると O(N^3)
# 選ばれる可能性のある数字は 000~999 までの1000通り
# それぞれを実現できるか調べられないか

from itertools import product

def main():
    N = int(input())
    S = input()

    ans = 0
    for num in product("0123456789", repeat=3):
        now = 0
        for i in S:
            if num[now] == i:
                now += 1
                if now == 3:
                    ans += 1
                    break
    print(ans)

if __name__ == "__main__":
    main()
