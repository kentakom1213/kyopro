#        B - 3人でカードゲームイージー
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc045/tasks/abc045_b
# ----------------------------------------

player = [list(input()[::-1]) for _ in range(3)]

turn = 0
while any(player):
    if not player[turn]:
        print("ABC"[turn])
        break
    nxt = player[turn].pop()
    turn = "abc".index(nxt)
