#                 B - 180°                
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc202/tasks/abc202_b
# ----------------------------------------

repl = {
    "0": "0",
    "1": "1",
    "6": "9",
    "8": "8",
    "9": "6",
}

S = input()

ans = "".join(map(lambda x:repl[x], reversed(S)))

print(ans)
