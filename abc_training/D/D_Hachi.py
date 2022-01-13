#               D - Hachi
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc181/tasks/abc181_d

# AC
# ----------------------------------------

from itertools import groupby

oct2sigits = []
for i in range(16, 104, 8):
    oct2sigits.append(set(str(i)))

oct3digits = [{"00", "01", "02"}]
for i in range(104, 1000, 8):
    chars = sorted(str(i))

    res = []
    for _, n in groupby(chars):
        res += [c + str(i) for i, c in enumerate(n)]

    oct3digits.append(set(res))

if __name__ == "__main__":
    S = input()

    isOK = False

    ls = len(S)
    if ls == 1:
        isOK = S == "8"
    elif ls == 2:
        S = set(S)
        isOK = S in oct2sigits
    else:
        S = sorted(S)
        res = []
        for _, n in groupby(S):
            res += [c + str(i) for i, c in enumerate(n)]
        
        isOK = any(oct <= set(res) for oct in oct3digits)


    print("Yes" if isOK else "No")