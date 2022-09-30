# https://atcoder.jp/contests/arc017/tasks/arc017_1

def yes():
    print("YES")
    exit()

def no():
    print("NO")
    exit()

N = int(input())

# 試し割り
i = 2
while i*i <= N:
  if N % i == 0:
    no()
  i += 1

yes()
  