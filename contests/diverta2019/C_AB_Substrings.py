#            C - AB Substrings            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/diverta2019/tasks/diverta2019_c

# バグが取れない
# ----------------------------------------

N = int(input())
ss = [input() for _ in range(N)]

cntAB = 0  # 元々あるAB
begin_with_B = end_with_A = 0  # Bで始まる文字列, Aで終わる文字列
B_and_A = 0  # 両方

for s in ss:
    cntAB += s.count("AB")
    if s[0] == "B" and s[-1] == "A":
        B_and_A += 1
    elif s[0] == "B":
        begin_with_B += 1
    elif s[-1] == "A":
        end_with_A += 1

if begin_with_B + end_with_A == 0:
    if B_and_A:
        print(cntAB + B_and_A - 1)
    else:
        print(cntAB)
else:
    print(cntAB + min(begin_with_B, end_with_A) + B_and_A)
