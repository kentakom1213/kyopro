
# これちょっとむずいな

N = int(input())
A = list(map(int, input().split()))

cnt = 0
res = []
for a in A:
    if res:
        if res[-1][0]==a:
            if res[-1][1] == a-1:
                res.pop()
                cnt -= a-1
            elif res[-1][1] >= a-1:
                res[-1][1] -= a-1
                cnt -= a-1
            else:
                res[-1][1] += 1
                cnt += 1
        else:
            res.append([a, 1])
            cnt += 1
    else:
        res.append([a, 1])
        cnt += 1
    print(cnt)
