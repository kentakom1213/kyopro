# https://atcoder.jp/contests/abc201/tasks/abc201_c

from itertools import product

s = input()
ans = 0

for tup in product(range(10), repeat=4):
    is_contain, isnot_contain = True, True
    for x in range(10):
        if s[x] == "o":
            is_contain &= x in tup
        elif s[x] == "x":
            isnot_contain &= x not in tup
    
    ans += is_contain and isnot_contain

print(ans)
