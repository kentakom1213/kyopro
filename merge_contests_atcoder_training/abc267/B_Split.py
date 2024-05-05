#                B - Split?               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc267/tasks/abc267_b
# ----------------------------------------

def yes():
    print("Yes")
    exit()

def no():
    print("No")
    exit()

S = input()

# 列
COLS = [
    [6],
    [3],
    [1, 7],
    [0, 4],
    [2, 8],
    [5],
    [9]
]

if S[0] == "1":
    no()

# ピンが残っている列を列挙
rem_cols = []
for i, c in enumerate(COLS):
    rem = False
    for pin in c:
        rem |= S[pin] == "1"
    if rem:
        rem_cols.append(i)

if not rem_cols:
    no()

# 離れているか判定
prev_col = rem_cols[0]
for i in rem_cols:
    if i - prev_col > 1:
        yes()
    prev_col = i

no()
