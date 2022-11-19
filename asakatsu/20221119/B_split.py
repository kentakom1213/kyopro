# https://atcoder.jp/contests/abc267/tasks/abc267_b

pins = list(map(int, list(input())))
cols = [
    [6],
    [3],
    [7, 1],
    [4, 0],
    [8, 2],
    [5],
    [9],
]

fall_down = [any(pins[i] for i in col) for col in cols]

for i in range(7):
    for j in range(i+1, 7):
        for k in range(j+1, 7):
            if pins[0] == 0 and (fall_down[i], fall_down[j], fall_down[k]) == (True, False, True):
                print("Yes")
                exit()
    
print("No")
