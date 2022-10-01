N = int(input())

ans = 0
now = 1*2*3*4*5*6*7*8*9*10

for i in range(10, 0, -1):
    cnt = 100
    while N >= now and cnt:
        N -= now
        cnt -= 1
        ans += 1
    now //= i

print(ans)
