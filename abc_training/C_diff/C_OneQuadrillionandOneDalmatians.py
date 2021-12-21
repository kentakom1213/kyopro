#  C - One Quadrillion and One Dalmatians
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc171/tasks/abc171_c

# 実装に時間かかりすぎ？
# https://drken1215.hatenablog.com/entry/2020/06/21/225500
# ↑かしこい

# AC
# ----------------------------------------

# aa..(n個)..a -> zz..(n個)..z は26^n個存在する
# sum(1->n-1, 26^k) = 26 * (26**n - 1) // 25

from string import ascii_lowercase

def binary_search(condition, min, max):
    while max - min > 1:
        mid = int((max + min) // 2)
        if condition(mid):
            min = mid
        else:
            max = mid
    return min


def sum_n(x): return 26 * (26**x - 1) // 25

def number_of_digit(n):
    cond = lambda x: sum_n(x) < n
    num = binary_search(cond, 0, 15) + 1
    return num


def main(n):
    d = number_of_digit(n)
    relative = n - sum_n(d-1) - 1

    res = [0] * d
    for i in range(d-1, -1, -1):
        res[i] = ascii_lowercase[relative % 26]
        relative //= 26

    print("".join(res))


if __name__ == "__main__":
    N = int(input())
    main(N)