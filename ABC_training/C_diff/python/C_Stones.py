#               C - Stones
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/tenka1-2019-beginner/tasks/tenka1_2019_c
# ----------------------------------------

# ex1)
#  8
#  "###...##" --> "......##"
# --> 3

# ex2)
#  8
#  "...###.." --> "...#####"
# --> 2

# OKのパターン
# 1: "." * N
# 2: "#" * N
# 3: "." * k + "#" * (N - k)

from itertools import groupby

if __name__ == "__main__":
    N = int(input())
    S = input()

    # 一番左にある"." / 一番右にある"#" を求める
    l_dot = r_sharp = -1
    for i, c in enumerate(S):
        if c == "." and l_dot == -1:
            l_dot = i
        if c == "#":
            r_sharp = i


    # rev_dot  : l_sharp以前の"." + l_sharp以降の"#"
    # rev_sharp: r_dot以降の"#" + r_dot以降の"."

    rev_dot = rev_sharp = 0
    for i, c in enumerate(S):
        if c == "." and i < l_sharp:
            rev_dot += 1
        if c == "#" and r_dot < i:
