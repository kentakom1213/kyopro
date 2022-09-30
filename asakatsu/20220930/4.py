# https://atcoder.jp/contests/abc166/tasks/abc166_e

"""
# 方針
i < j を満たすi, jについて考えると、
    |i-j| = Ai + Aj
<=>  j-i = Ai + Aj
<=> Ai+i = j-Aj

Ai+iの個数をdictに保存しながらj-Ajの個数を数え上げる。
"""

from collections import defaultdict

N = int(input())
A = list(map(int, input().split()))

ans = 0
Ai_i = defaultdict(int)

for j in range(N):
    ans += Ai_i[j - A[j]]
    Ai_i[A[j] + j] += 1

print(ans)