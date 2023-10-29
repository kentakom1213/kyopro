#            D - Opposite
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc197/tasks/abc197_d
# ----------------------------------------

N,x,y=(complex(*map(int,t.split()))/2 for t in open(0))
p=(x-y)*2.718281**(3.141592j/N)+x+y
print(p.real,p.imag)
