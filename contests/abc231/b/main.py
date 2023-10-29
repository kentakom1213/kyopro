
N = int(input())
map = {}

for i in range(N):
    name = input()
    if name not in map:
        map[name] = 1
    else:
        map[name] += 1

res = sorted(map.items(), key=lambda x: x[1], reverse=True)

print(res[0][0])