#                B - -- - B               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc112/tasks/arc112_b
# ----------------------------------------

"""comment
## 解法
1. ちょうどC円を使って作れる整数を考える
2. ちょうどC-1円を使って作れる整数を考える
3. 数える

- Cが奇数のとき、偶数のときで場合分け

[解説](https://atcoder.jp/contests/arc112/editorial/725)
"""

B, C = map(int, input().split())

if B and C == 1:
    print(2)
    exit()

def get_range(B, C):
    """BからC回ちょうどで作れる数の範囲を返す"""
    n = C // 2
    if C % 2 == 0:
        return B-n, B+n-1
    else:
        return -B-n, -B+n

# C円ちょうどで作れる数
a, b = get_range(B, C)

# C-1円ちょうどで作れる数
c, d = get_range(B, C-1)

# 重複部分を排除して数える
ans = (b - a + 1) + (d - c + 1) - max(0, min(b, d) - max(a, c) + 1)
print(ans)
