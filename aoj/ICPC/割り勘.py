#                   割り勘                   
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2944&lang=jp
# ----------------------------------------

def solve():
    n, m = map(int, input().split())

    if n == m == 0:
        exit()

    A = [int(v) for v in input().split()]

    t = m // n
    res = 0
    for a in A:
        res += min(t, a)
    
    print(res)
    

def main():
    while True:
        solve()

main()
