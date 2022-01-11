#               C - Stones
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/tenka1-2019-beginner/tasks/tenka1_2019_c

# 頭いいなあ...

# AC
# ----------------------------------------

# これでは解けない気がする

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

# if __name__ == "__main__":
#     N = int(input())
#     S = input()

#     # 一番左にある"." / 一番右にある"#" を求める
#     l_dot = r_sharp = -1
#     for i, c in enumerate(S):
#         if c == "." and l_dot == -1:
#             l_dot = i
#         if c == "#":
#             r_sharp = i


#     # rev_dot  : l_sharp以前の"." + l_sharp以降の"#"
#     # rev_sharp: r_dot以降の"#" + r_dot以降の"."

#     rev_dot = rev_sharp = 0
#     for i, c in enumerate(S):
#         if c == "." and i < l_sharp:
#             rev_dot += 1
#         if c == "#" and r_dot < i:


# 解説
# https://img.atcoder.jp/tenka1-2019/editorial.pdf

# S[:i].count("#") + S[i:].count(".") の最小値を求めれば良い

if __name__ == "__main__":
    N = int(input())
    S = input()

    l_sharp = 0
    r_dot = S.count(".")
    res = l_sharp + r_dot  # 答え
    for i, c in enumerate(S):
        if c == "#":
            l_sharp += 1
        else:
            r_dot -= 1
        
        # 更新
        res = min(res, l_sharp + r_dot)
    
    print(res)