#              文字列の全探索 5
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/237

# AC
# ----------------------------------------

N = int(input())
S = [input() for _ in range(N)]

def is_palindrome(s):
    for i in range(len(s) // 2 + 1):
        if s[i] != s[-i-1]:
            return False
    else:
        return True

print(len( list(filter(is_palindrome, S)) ))