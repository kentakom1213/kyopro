#        A - Arithmetic Sequence
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc123/tasks/arc123_a

# arcの灰diffは絶対盛ってる


# ----------------------------------------

a1, a2, a3 = map(int, input().split())

ans = 1e30

# arrange left
if a1 <= 2*a2 - a3:
    ans = min(
        ans,
        2*a2 - a3 - a1
    )

# arrange center
center = (a3 + a1) // 2
if (a3 + a1)%2 == 0 and a2 <= center:
    ans = min(
        ans,
        center - a2
    )

# arrange right
if a3 <= 2*a2 - a1:
    ans = min(
        ans,
        2*a2 - a1 - a3
    )

print(ans)
