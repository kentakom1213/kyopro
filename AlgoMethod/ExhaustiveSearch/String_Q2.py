#              文字列の全探索 2
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/227
# ----------------------------------------

S = input()

for i in range(len(S)//2):
	if S[i] != S[-1-i]:
		print("No")
		break
else:
	print("Yes")