#            A - AB Palindrome            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc145/tasks/arc145_a
# ----------------------------------------

def solve(s):
    print(s)

    # もともと回文
    if s == s[::-1]:
        return True

    x, y = s[0], s[-1]

    # A..B のとき
    if x+y == "AB":
        return False
    
    # A..A, B..B のとき
    if x+y in ["AA", "BB"]:
        return solve(s[1:-1])
    
    # B..A のとき
    return solve("AB" + s[2:]) or solve(s[:-2] + "AB")

if __name__ == "__main__":
    N = int(input())
    S = input()
    print("Yes" if solve(S) else "No")
