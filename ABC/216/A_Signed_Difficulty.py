# A - Signed Difficulty
# AC

# # input
f = float(input())

def SD(f):
    X = int(f)
    Y = int(10*(f - X))  # 丸め誤差怖いねえ

    if 0 <= Y <= 2:
        print(f"{X}-")
    elif 3 <= Y <= 6:
        print(f"{X}")
    else:
        print(f"{X}+")

SD(f)

# # test
# for i in range(160):
#     f = i / 10
#     print(f)
#     SD(f)
