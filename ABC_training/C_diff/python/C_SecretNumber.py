#           C - Secret Number
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc201/tasks/abc201_c

# AC (解説)
# ----------------------------------------


# S = input()
# O, X, Q = S.count("o"), S.count("x"), S.count("?")

# if O > 4: print(0)
# elif O == 4: print(1)
# elif O == 3:
#     nonQ = 3 * 12
#     Q1 = Q * 24  # 4!
#     print(nonQ + Q1)
# elif O == 2:
#     nonQ = 6  # 4! / (2! * 2!)
#     Q1 = Q * 12  # 4! / 2!
#     Q2 = Q * (Q - 1) * 24
#     print(nonQ + Q1 + Q2)
# elif O == 1:
#     nonQ = 1
#     Q1 = Q * 4
#     Q2 = Q * (Q - 1) * 2 * 12
#     Q3 = Q * (Q - 1) * (Q - 2) * 24
#     print(nonQ + Q1 + Q2 + Q3)
# else:
#     Q1 = Q
#     Q2 = Q * (Q - 1) * 14  # 2 * (4! / 3!) + (4! / (2! * 2!))
#     Q3 = Q * (Q - 1) * (Q - 2) * 3 * 12  # 3 * (4! / 2!)
#     Q4 = Q * (Q - 1) * (Q - 2) * (Q - 3)
#     print(Q1 + Q2 + Q3 + Q4)

# -> ????

# どうせ 10^4 通りしかないから、全探索でOK
from itertools import product

S = input()

def is_match(num, pattern):
    nums = map(int, num)
    O = pattern.count("o")

    for n in nums:
        if pattern[n] == "x":
            return False
        elif pattern[n] == "o":
            O -= 1
            pattern = pattern[:n] + "?" + pattern[n+1:]
    
    if O > 0:
        return False
    return True


counter = 0

secret = ["0123456789"] * 4
for num in product(*secret):
    if is_match(num, S):
        # print("".join(num), is_match(num, S))
        counter += 1

print(counter)

# 計算量は O(1) かな