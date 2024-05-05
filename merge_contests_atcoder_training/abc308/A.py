S = list(map(int, input().split()))

cur = 0
is_ok = True

for s in S:
    is_ok &= cur <= s
    is_ok &= 100 <= s <= 675
    is_ok &= s % 25 == 0

    cur = s

print("Yes" if is_ok else "No")
