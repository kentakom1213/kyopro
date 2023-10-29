N = int(input())
S = input()

is_takahashi = False
aoki = 0
takahashi = 0

for c in S:
    if c == "T":
        takahashi += 1
    else:
        aoki += 1

    if N % 2 == 0 and N // 2 == takahashi:
        is_takahashi = True
        break
    if N % 2 == 0 and N // 2 == aoki:
        break

if aoki < takahashi:
    print("T")
elif aoki > takahashi:
    print("A")
else:
    if is_takahashi:
        print("T")
    else:
        print("A")
