#          A - Legendary Players          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc319/tasks/abc319_a
# ----------------------------------------

rate = [
    "tourist 3858",
    "ksun48 3679",
    "Benq 3658",
    "Um_nik 3648",
    "apiad 3638",
    "Stonefeang 3630",
    "ecnerwala 3613",
    "mnbvmar 3555",
    "newbiedmy 3516",
    "semiexp 3481",
]

name = input()

for s in rate:
    if name in s:
        print(s.split()[1])
