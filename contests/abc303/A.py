N = int(input())
S = input()
T = input()

is_ok = True
for x, y in zip(S, T):
    if x + y in "l1l":
        continue
    if x + y in "0o0":
        continue
    if x == y:
        continue
    is_ok = False

print("Yes" if is_ok else "No")
