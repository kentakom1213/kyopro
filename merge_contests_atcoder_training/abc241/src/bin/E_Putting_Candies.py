#           E - Putting Candies           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc241/tasks/abc241_e
# ----------------------------------------

"""

## 方針
- ループ検出

"""


n, k = map(int, input().split())
a = list(map(int, input().split()))

loop_begin = 0
loop_end = 0
loop_len = 0
loop_cand = 0

mp = {}
i = cur = 0
while True:
    if i >= k:
        print(cur)
        exit()
    
    if (cur % n) in mp:
        loop_begin = mp[cur%n][0]
        loop_end = i
        loop_len = loop_end - loop_begin
        loop_cand = cur - mp[cur%n][1]
        break

    # シミュレート
    mp[cur%n] = (i, cur)
    cur += a[cur%n]
    i += 1

print(mp)
print(f"""{loop_begin=}\n{loop_end=}\n{loop_len=}\n{loop_cand=}""")

loop_time = (k - loop_begin) // loop_len
loop_rem = k % loop_len - loop_begin
ans = mp[loop_begin][1] + loop_time * loop_cand

# 残りをシミュレート
for _ in range(loop_rem):
    ans += a[cur%n]
    cur = (cur + a[cur%n]) % n

print(ans)
