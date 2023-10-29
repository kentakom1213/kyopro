#            A - AB Palindrome            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc145/tasks/arc145_a
# ----------------------------------------

def yes():
    print("Yes")
    exit()

def no():
    print("No")
    exit()

# --------------
N = int(input())
S = input()

if S == "BA" or (S[0]+S[-1] == "AB"):
    no()
else:
    yes()
