#               B - 天下一リテラル               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/tenka1-2015-qualb/tasks/tenka1_2015_qualB_b
# ----------------------------------------

expr = input()
exec(f"res = {expr}")

if type(res) == type({}):
    print("dict")
else:
    print("set")

