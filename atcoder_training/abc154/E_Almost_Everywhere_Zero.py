#        E - Almost Everywhere Zero       
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc154/tasks/abc154_e

# 参考
# https://atcoder.jp/contests/abc154/submissions/9991364
# ----------------------------------------

from functools import lru_cache

# メモ化再帰をする関数
@lru_cache
def f(n, k):
    if n < 10:
        if k == 0:
            return 1
        elif k == 1:
            return n
        return 0
    
    q, r = divmod(n, 10)
    res = 0
    if k >= 1:
        res += f(q, k-1) * r
        res += f(q-1, k-1) * (9-r)
    res += f(q, k)
    return res

def main():
    N = int(input())
    K = int(input())
    print(f(N, K))

main()
