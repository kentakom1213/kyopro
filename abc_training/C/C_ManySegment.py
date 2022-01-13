#            C - Many Segments
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc207/tasks/abc207_c

# AC
# ----------------------------------------

N = int(input())
sections = []
for _ in range(N):
    t, l, r = map(int, input().split())
    if l == r:
        if t == 1:
            sections.append((l, r))
    else:
        if t == 1:
            sections.append((l, r))
        elif t == 2:
            sections.append((l, r-0.1))
        elif t == 3:
            sections.append((l+0.1, r))
        elif t == 4:
            sections.append((l+0.1, r-0.1))

counter = 0
for i in range(N):
    for j in range(i+1, N):
        if sections[i][1] < sections[j][0] or sections[i][0] > sections[j][1]:
            continue
        counter += 1
   
print(counter)
