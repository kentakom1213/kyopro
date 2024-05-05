
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]
def exprint(x): print(*x, sep="\n")

N, A, B = map(int, input().split())
P, Q, R, S = [int(v)-1 for v in input().split()]
A -= 1
B -= 1

H, W = Q-P+1, S-R+1
field = init_array(H, W, ".")

for i in range(P, Q+1):
    # print(i, i-A+B)
    if R <= i-A+B <= S+1:
        try:
            field[i-P][i-R-A+B] = "#"
        except:
            pass

for i in range(P, Q+1):
    # print(i, -i+A+B)
    if R <= -i+A+B <= S+1:
        try:
            field[i-P][-i-R+A+B] = "#"
        except:
            pass

for row in field:
    print("".join(row))