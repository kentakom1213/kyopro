#           C - 辞書式順序ふたたび
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc009/tasks/abc009_3

# これむずいのでパス
# ----------------------------------------

from string import ascii_lowercase
def num(char): return ascii_lowercase.index(char)
def get_chars(string): return sorted(list(string), key=num)

# input
N, K = map(int, input().split())
S = input()

# solve
def can_put_top(K, string, top):
    chars = get_chars(string)

    for s in string:
        pass



print(can_put_top(K, S, S[1]))
