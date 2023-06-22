#        Problem A: ICPC 得点集計ソフトウェア       
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1147&lang=jp
# ----------------------------------------

def main():
    while True:
        n = int(input())
        if n == 0:
            return
        res = sum(sorted(int(input()) for _ in range(n))[1:-1]) // (n - 2)
        print(res)

main()
