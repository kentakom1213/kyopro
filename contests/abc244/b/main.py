
N = int(input())
T = input()

res = 0
d = 1

for c in T:
    if c == "S":
        res += d
    else:
        d *= -1j

print(int(res.real), int(res.imag))
