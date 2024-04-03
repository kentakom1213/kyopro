N = int(input())

# 列挙
reps = [int("1" * i) for i in range(1, 111)]

sums = set()

for i in range(100):
    for j in range(i, 100):
        for k in range(j, 100):
            s = reps[i] + reps[j] + reps[k]
            sums.add(s)

ssums = sorted(sums)

print(ssums[N - 1])
