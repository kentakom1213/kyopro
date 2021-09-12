#                A - おつり
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_a

# AC
# ----------------------------------------


# input
N = int(input())

# solve
coins = (500, 100, 50, 10, 5, 1)

def get_change(N):
    rest = 1000 - N

    count = 0
    for c in coins:
        while rest:
            if rest >= c:
                rest -= c
                count += 1
            else:
                break

    return count

print(get_change(N))
