
c = "0123456789ABCDEF"

N = int(input())

ans = ""
while N:
    ans = c[N % 16] + ans
    N //= 16

if ans == "":
    print("00")
elif len(ans) == 1:
    print("0" + ans)
else:
    print(ans)