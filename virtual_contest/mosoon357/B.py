# https://atcoder.jp/contests/abc223/tasks/abc223_b

S = input()

l = []
for i in range(len(S)):
    l.append(S[i:] + S[:i])

l.sort()

print(l[0])
print(l[-1])
