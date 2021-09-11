#       C - Optimal Recommendations 
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/indeednow-finala-open/tasks/indeednow_2015_finala_c

# 一旦パス、企業コンテストは難しいのか？
# pythonだと絶対TLEなる気がする
# ----------------------------------------

from operator import itemgetter

def exprint(x): print(*x, sep="\n")

def getLCS(company, seeker):




if __name__ == "__main__":

    N, M = map(int, input().split())
    companies = [list(map(int, input().split())) for _ in range(N)]
    seekers = [list(map(int, input().split())) for _ in range(M)]

    companies.sort(key=itemgetter(3), reverse=True)
